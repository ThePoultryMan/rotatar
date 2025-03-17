use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};

use async_channel::Receiver;
use iced::{
    futures::{SinkExt, Stream},
    stream, widget, Background, Subscription,
};

use crate::{
    config::Config,
    message::Message,
    state::{to_2d_index, State},
};

pub struct App {
    config: Config,
    current_image: usize,
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
            current_image: to_2d_index(sections.0 / 2, sections.1 / 2, sections.0),
            receiver: Arc::new(receiver),
            state: Arc::new(Mutex::new(State::new(screen_size, sections))),
            background_color,
        }
    }

    pub fn update(&mut self, message: Message) {
        match message {
            Message::CurrentImageChanged => {
                if let Ok(state) = self.state.lock() {
                    self.current_image = state.current_image();
                }
            }
            Message::Ready(sender) => {
                let _ = sender.send_blocking(self.receiver.clone());
            }
            Message::SpeakingStateChange(speaking) => {
                if let Ok(mut state) = self.state.lock() {
                    state.set_speaking(speaking);
                }
            }
        }
    }

    pub fn view(&self) -> iced::Element<'_, Message> {
        if let Ok(state) = self.state.lock() {
            let row = { widget::row![widget::image(self.get_current_image(&state))] };
            widget::container(row)
                .style(|_| {
                    widget::container::Style::default()
                        .background(Background::Color(self.background_color))
                })
                .center_x(iced::Length::Fill)
                .center_y(iced::Length::Fill)
                .width(iced::Length::Fill)
                .height(iced::Length::Fill)
                .into()
        } else {
            widget::container(widget::text(
                "critical error encountered, restart pretty please.",
            ))
            .center_x(iced::Length::Fill)
            .center_y(iced::Length::Fill)
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .into()
        }
    }

    fn get_current_image(&self, state: &State) -> &PathBuf {
        if state.speaking() {
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
            let _ = output.send(Message::Ready(sender.clone())).await;
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
