use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};

use async_channel::{Receiver, Sender};
use iced::{
    Background, Subscription, Task,
    futures::{SinkExt, Stream},
    stream, widget,
};
use rotatar_backend::{
    Message, State, arctex,
    audio::{self, AudioMessage, AudioStatus},
    set_state,
};
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
    config: Arc<Mutex<Config>>,
    receiver: Arc<Receiver<Message>>,
    state: Arc<Mutex<State>>,
    background_color: iced::Color,
}

impl App {
    pub fn new(
        config: Config,
        background_color: iced::Color,
        receiver: Receiver<Message>,
        message_sender: Sender<Message>,
        audio_sender: Sender<AudioMessage>,
        audio_receiver: Receiver<AudioMessage>,
    ) -> Self {
        let screen_size = config.screen_information().size();
        let sections = config.sections();
        Self {
            config: arctex!(config),
            receiver: Arc::new(receiver),
            state: arctex!(State::new(
                message_sender,
                screen_size,
                sections,
                audio_sender,
                audio_receiver
            )),
            background_color,
        }
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::SetupAudio(mut audio_handler) => {
                audio_handler.update_input_devices();
                let index = audio_handler.current_input_index();
                return Task::future(audio::handle_audio(audio_handler, index));
            }
            Message::UpdateAudioStatus(audio_status, audio_handler) => {
                if audio_status == AudioStatus::Polling {
                    if let Some(audio_handler) = audio_handler {
                        let index = audio_handler.current_input_index();
                        return Task::future(audio::wait_for_audio(audio_handler, index));
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

    pub fn config(&self) -> Arc<Mutex<Config>> {
        self.config.clone()
    }

    fn get_current_image(&self, state: &State) -> PathBuf {
        match self.config.lock() {
            Ok(config) => {
                if state.is_speaking() {
                    config
                        .speaking_images()
                        .get(state.current_image())
                        .expect("There should be an image")
                        .clone()
                } else {
                    config
                        .idle_images()
                        .get(state.current_image())
                        .expect("There should be an image")
                        .clone()
                }
            }
            Err(error) => todo!("{error}"),
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
