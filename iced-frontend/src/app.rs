use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use async_channel::Receiver;
use iced::{
    Background, Subscription, Task,
    futures::{SinkExt, Stream},
    stream, widget,
};
use rotatar_backend::{AudioHandler, AudioResult, AudioStatus, Message, State, set_state};
use rotatar_types::Config;

macro_rules! audio_section {
    ($state:expr) => {
        widget::column![
            {
                if *$state.audio_status() == AudioStatus::Ready {
                    widget::text!("Audio input connected")
                } else {
                    widget::text!("No audio input")
                }
            },
            { widget::progress_bar(0.0..=1.0, $state.sensitivity()) }
        ]
    };
}

pub struct App {
    config: Config,
    receiver: Arc<Receiver<Message>>,
    state: Arc<Mutex<State>>,
    background_color: iced::Color,
}

impl App {
    pub fn new(config: Config, background_color: iced::Color, receiver: Receiver<Message>) -> Self {
        let screen_size = config.screen_size();
        let sections = config.sections();
        Self {
            config,
            receiver: Arc::new(receiver),
            state: Arc::new(Mutex::new(State::new(screen_size, sections))),
            background_color,
        }
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::SetupAudio(mut audio_handler) => {
                audio_handler.update_input_devices();
                return Task::future(handle_audio(audio_handler));
            }
            Message::UpdateAudioStatus(audio_status) => {
                if let AudioStatus::Polling { audio_handler } = audio_status {
                    if let Some(audio_handler) = audio_handler {
                        return Task::future(wait_for_audio(audio_handler));
                    }
                } else {
                    set_state!(self.state, set_audio_status, audio_status);
                }
            }
            Message::OutsideListenerReady(sender) => {
                let _ = sender.send_blocking(self.receiver.clone());
            }
            Message::SensitivityChanged(sensitivity) => {
                set_state!(self.state, set_sensitivity, sensitivity);
            }
            _ => {}
        }
        Task::none()
    }

    pub fn view(&self) -> iced::Element<'_, Message> {
        if let Ok(state) = self.state.lock() {
            let row = widget::row![widget::image(self.get_current_image(&state))];
            let column = widget::column![row, audio_section!(state)];
            widget::center(column)
                .style(|_| {
                    widget::container::Style::default()
                        .background(Background::Color(self.background_color))
                })
                .width(iced::Length::Fill)
                .height(iced::Length::Fill)
                .into()
        } else {
            widget::center(widget::text(
                "critical error encountered, restart pretty please.",
            ))
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .into()
        }
    }

    fn get_current_image(&self, state: &State) -> &PathBuf {
        if state.is_speaking() {
            self.config
                .speaking_images()
                .get(state.current_image())
                .expect("There should be an image")
        } else {
            self.config
                .idle_images()
                .get(state.current_image())
                .expect("There should be an image")
        }
    }

    fn state_updater() -> impl Stream<Item = Message> {
        stream::channel(100, |mut output| async move {
            let (sender, receiver) = async_channel::unbounded();
            let _ = output
                .send(Message::OutsideListenerReady(sender.clone()))
                .await;
            let outside_receiver = receiver.recv().await.expect("Receiving error");

            loop {
                if let Ok(message) = outside_receiver.recv().await {
                    let _ = output.send(message).await;
                }
            }
        })
    }

    pub fn subscription(&self) -> Subscription<Message> {
        Subscription::run(Self::state_updater)
    }

    /// Gets a clone of this app's [Arc](std::sync::Arc)<[Mutex](std::sync::Mutex)<[State](crate::state::State)>>
    pub fn state(&self) -> Arc<Mutex<State>> {
        self.state.clone()
    }
}

async fn handle_audio(mut audio_handler: AudioHandler) -> Message {
    if audio_handler.set_current_input_device(0) {
        let result = audio_handler.play().await;
        // AudioResult::Closed is the only time that everything is ok.
        if result.result() == AudioResult::Closed
            || result.result() == AudioResult::DeviceNotAvailable
        {
            let audio_handler = result.audio_handler();
            let _ = audio_handler
                .sender()
                .send(Message::UpdateAudioStatus(AudioStatus::Closed))
                .await;
            Message::SetupAudio(audio_handler)
        } else {
            panic!(
                "AudioResult with unhandled error occurred:\n{}",
                result.result()
            );
        }
    } else {
        Message::UpdateAudioStatus(AudioStatus::Polling {
            audio_handler: Some(audio_handler),
        })
    }
}

async fn wait_for_audio(mut audio_handler: AudioHandler) -> Message {
    thread::sleep(Duration::from_millis(75));
    audio_handler.update_input_devices();
    handle_audio(audio_handler).await
}
