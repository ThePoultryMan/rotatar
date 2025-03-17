use std::time::Instant;

use async_channel::Sender;
use cpal::{
    traits::{DeviceTrait, HostTrait, StreamTrait},
    BuildStreamError, Device, PlayStreamError, SupportedStreamConfig, SupportedStreamConfigsError,
};
use rustfft::{num_complex::Complex, FftPlanner};
use thiserror::Error;

use crate::{message::Message, util::arctex};

pub struct AudioHolder {
    device: Device,
    supported_config: SupportedStreamConfig,
    sender: Sender<Message>,
}

#[derive(Debug, Error)]
pub enum AudioError {
    #[error("No input device could be found")]
    NoInputDevice,
    #[error("No SupportedStreamConfigRange was found")]
    NoSupportedConfigRange,
    #[error("SupportedStreamConfigsError")]
    SupportedStreamConfigs(#[from] SupportedStreamConfigsError),
    #[error("Error while building audio input stream")]
    BuildStreamError(#[from] BuildStreamError),
    #[error("Error while playing stream")]
    PlayStreamError(#[from] PlayStreamError),
}

impl AudioHolder {
    pub fn new(sender: Sender<Message>) -> Result<Self, AudioError> {
        if let Some(input_device) = cpal::default_host().default_input_device() {
            let mut supported_configs_range = input_device.supported_input_configs()?;
            if let Some(supported_config) = supported_configs_range.next() {
                Ok(Self {
                    device: input_device,
                    supported_config: supported_config.with_max_sample_rate(),
                    sender,
                })
            } else {
                Err(AudioError::NoSupportedConfigRange)
            }
        } else {
            Err(AudioError::NoInputDevice)
        }
    }

    /// Creating and running the stream does not block the thread, due to cpal's behavior.
    pub async fn stream(self) -> Result<(), AudioError> {
        let sampling_rate = self.supported_config.sample_rate().0;

        // We store these in here so the exist between all calls of the data callback.
        let last_time = arctex!(Instant::now());
        let sensitivity = arctex!(0.0);
        let last_sensitivity = arctex!(0.0);

        let moved_sender = self.sender.clone();
        match self.device.build_input_stream(
            &self.supported_config.config(),
            move |data: &[f32], _: &cpal::InputCallbackInfo| {
                if let (Ok(mut last_time), Ok(mut sensitivity), Ok(mut last_sensitivity)) =
                    (last_time.lock(), sensitivity.lock(), last_sensitivity.lock())
                {
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
                    let max_magnitude = *magnitudes.iter().max().unwrap_or(&0);

                    // If the maximum magnitude is greater than the "speaking threshold," sensitivity
                    // is set to 1.0. If not, the sensitivity is decreased at a rate of 3.0 sensitivity/second.
                    // Calculated using the delta found before.
                    if max_magnitude > 6 {
                        *sensitivity = 1.0;
                    } else {
                        *sensitivity = (*sensitivity - (3.0 * delta)).max(0.0);
                    }

                    // If the current sensitivity does not equal the last sensitivity, send an update
                    // so that the state updates.
                    if *last_sensitivity != *sensitivity {
                        // We can safely ignore this result because if the channel closes, the stream
                        // will be dropped.
                        let _ = moved_sender.send_blocking(Message::SensitivityChanged(*sensitivity));
                    }
                    *last_sensitivity = *sensitivity;
                }
            },
            move |error| {
                panic!("{:#?}", error);
            },
            None,
        ) {
            Ok(stream) => {
                stream.play()?;
                while !self.sender.is_closed() {}
                Ok(())
            }
            Err(error) => Err(AudioError::BuildStreamError(error)),
        }
    }
}
