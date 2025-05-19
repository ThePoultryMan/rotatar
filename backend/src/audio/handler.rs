use std::{fmt::Debug, time::Instant};

use async_channel::Sender;
use cpal::{
    Device, Host, StreamConfig,
    traits::{DeviceTrait, HostTrait, StreamTrait},
};
use rotatar_types::AudioConfig;
use rustfft::{FftPlanner, num_complex::Complex};

use crate::{arctex, message::Message};

use super::{AudioHandlerResult, AudioStatus, error::AudioResult};

pub struct AudioHandler {
    host: Host,
    sender: Sender<Message>,
    input_devices: Vec<Device>,
    current_input_index: usize,
    config: Option<StreamConfig>,
    audio_config: AudioConfig,
}

impl AudioHandler {
    pub fn new(sender: Sender<Message>, audio_config: AudioConfig) -> Self {
        Self {
            host: cpal::default_host(),
            sender,
            input_devices: Vec::new(),
            current_input_index: 0,
            config: None,
            audio_config,
        }
    }

    pub fn sender(&self) -> &Sender<Message> {
        &self.sender
    }

    /// Attempts to update internal list of input devices.
    /// Returns the current internal list of input devices, regardless of success.
    pub fn update_input_devices(&mut self) -> &Vec<Device> {
        if let Ok(input_devices) = self.host.input_devices() {
            self.input_devices = input_devices.collect();
        }
        self.sender.send_blocking(Message::AudioDevicesChanged(
            self.input_devices
                .iter()
                .filter_map(|device| device.name().ok())
                .collect(),
        )).unwrap();
        &self.input_devices
    }

    /// Selects the first device in the internal list of input devices that contains "default" in
    /// the device name. If no device meets those conditions, then no change is made to the selected
    /// device index.
    pub fn select_default_device(&mut self) -> bool {
        for (index, device) in self.input_devices.iter().enumerate() {
            if let Ok(name) = device.name() {
                if name.to_ascii_lowercase().contains("default") {
                    return self.set_current_input_device(index);
                }
            }
        }
        false
    }

    /// Sets the current input device by the index in respect to the internal list of input devices.
    /// Also updates the StreamConfig.
    ///
    /// Returns true if config was set, and false if config was not set.
    pub fn set_current_input_device(&mut self, index: usize) -> bool {
        self.current_input_index = index;
        if let Some(device) = self.input_devices.get(index) {
            if let Ok(mut supported_input_configs) = device.supported_input_configs() {
                if let Some(supported_input_config) = supported_input_configs.next() {
                    self.config = Some(supported_input_config.with_max_sample_rate().config());
                    println!("Device '{index}' has been selected");
                    return true;
                }
            }
        }
        self.config = None;
        false
    }

    /// Consumes the AudioHolder, returns a wrapper containing information about the exit when the
    /// future resolves.
    ///
    /// The future resolves when:
    /// 1. No config was found.
    /// 2. An error occurred while playing the stream.
    pub async fn play(self) -> AudioHandlerResult {
        if let Some(ref config) = self.config {
            let sampling_rate = config.sample_rate.0;
            // We store these in here so the exist between all calls of the data callback.
            let last_time = arctex!(Instant::now());
            let sensitivity = arctex!(0.0);
            let last_sensitivity = arctex!(0.0);
            let error = arctex!(None);
            let error_clone = error.clone();
            let data_callback_sender = self.sender.clone();
            let error_callback_sender = self.sender.clone();
            let _ = self
                .sender
                .send(Message::UpdateAudioStatus(AudioStatus::Ready))
                .await;
            match self.input_devices[self.current_input_index].build_input_stream(
                config,
                move |data: &[f32], _: &cpal::InputCallbackInfo| {
                    if let (Ok(mut last_time), Ok(mut sensitivity), Ok(mut last_sensitivity)) = (
                        last_time.lock(),
                        sensitivity.lock(),
                        last_sensitivity.lock(),
                    ) {
                        // Get the time since the last call.
                        let delta = Instant::now().duration_since(*last_time).as_secs_f32();
                        *last_time = Instant::now();
                        // Do FFT
                        let mut planner = FftPlanner::new();
                        let fft = planner.plan_fft_inverse(data.len());
                        let mut buffer: Vec<Complex<f32>> = Vec::new();
                        for item in data {
                            buffer.push(Complex { re: *item, im: 0.0 });
                        }
                        fft.process(&mut buffer);
                        // Get maximum magnitude of FFT between 20hz and 20000hz
                        let start = (20.0 * buffer.len() as f32 / sampling_rate as f32) as usize;
                        let end = (20000.0 * buffer.len() as f32 / sampling_rate as f32) as usize;
                        let mut magnitudes = Vec::with_capacity(end - start);
                        for item in buffer.iter().take(end).skip(start) {
                            magnitudes.push((item.norm_sqr() as f64).sqrt() as i32 + 1);
                        }
                        let maximum_magnitude = *magnitudes
                            .iter()
                            .max()
                            .unwrap_or(&0)
                            .min(&self.audio_config.max_magnitude());
                        data_callback_sender
                            .send_blocking(Message::MagnitudeChanged(maximum_magnitude))
                            .unwrap();
                        // If the maximum magnitude is greater than the "speaking threshold," sensitivity
                        // is set to 1.0. If not, the sensitivity is decreased at a rate of 3.0 sensitivity/second
                        // Calculated using the delta found before.
                        if maximum_magnitude > self.audio_config.magnitude_threshold() {
                            *sensitivity = 1.0;
                        } else {
                            *sensitivity = (*sensitivity - (3.0 * delta)).max(0.0);
                        }
                        // If the current sensitivity does not equal the last sensitivity, send an update
                        // so that the state updates.
                        if *last_sensitivity != *sensitivity {
                            // We can safely ignore this result because if the channel closes, the stream
                            // will be dropped.
                            let _ = data_callback_sender
                                .send_blocking(Message::SensitivityChanged(*sensitivity));
                            *last_sensitivity = *sensitivity;
                        }
                    }
                },
                move |stream_error| {
                    let _ = error_callback_sender.send_blocking(Message::SensitivityChanged(0.0));
                    if let Ok(mut error) = error_clone.lock() {
                        *error = Some(stream_error.into())
                    } else {
                        panic!("Critical error occurred within AudioHolder::stream error_callback");
                    }
                },
                None,
            ) {
                Ok(stream) => {
                    // If anything goes wrong during reading, return result
                    if let Ok(()) = stream.play() {
                        while !self.sender.is_closed() {
                            if let Ok(error) = error.clone().lock() {
                                if let Some(error) = *error {
                                    return AudioHandlerResult {
                                        audio_handler: self,
                                        result: error,
                                    };
                                }
                            } else {
                                panic!(
                                    "Critical error occurred within AudioHolder::stream keep alive while loop"
                                );
                            }
                        }
                        AudioHandlerResult {
                            audio_handler: self,
                            result: AudioResult::Closed,
                        }
                    } else {
                        AudioHandlerResult {
                            audio_handler: self,
                            result: AudioResult::Play,
                        }
                    }
                }
                Err(_) => AudioHandlerResult {
                    audio_handler: self,
                    result: AudioResult::BuildStreamError,
                },
            }
        } else {
            AudioHandlerResult {
                audio_handler: self,
                result: AudioResult::NoConfig,
            }
        }
    }
}

impl Clone for AudioHandler {
    fn clone(&self) -> Self {
        Self {
            host: cpal::default_host(),
            sender: self.sender.clone(),
            input_devices: self.input_devices.clone(),
            current_input_index: self.current_input_index,
            config: self.config.clone(),
            audio_config: self.audio_config,
        }
    }
}

impl Debug for AudioHandler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let input_devices: Vec<String> = self
            .input_devices
            .iter()
            .map(|device| device.name().unwrap_or(String::from("error getting name")))
            .collect();
        f.debug_struct("AudioHandler")
            .field("host", &"Not Shown")
            .field("sender", &self.sender)
            .field("input_devices", &input_devices)
            .field("current_input_index", &self.current_input_index)
            .field("config", &self.config)
            .finish()
    }
}
