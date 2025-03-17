use std::{
    path::PathBuf,
    sync::{Arc, Mutex},
};

use async_channel::Receiver;
use iced::{
    futures::{SinkExt, Stream},
    stream, widget::{self, text}, Background, Color, Subscription,
};

use crate::{
    config::Config,
    message::Message,
    state::{to_2d_index, State},
};

pub struct App {
    images: Vec<PathBuf>,
    current_image: usize,
    receiver: Arc<Receiver<Message>>,
    state: Arc<Mutex<State>>,
}

impl App {
    pub fn new(config: Config, receiver: Receiver<Message>) -> Self {
        let screen_size = config.screen_size();
        let sections = config.sections();
        Self {
            images: config.images(),
            current_image: to_2d_index(sections.0 / 2, sections.1 / 2, sections.0),
            receiver: Arc::new(receiver),
            state: Arc::new(Mutex::new(State::new(screen_size, sections))),
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
            },
            Message::SpeakingStateChange(speaking) => {
                if let Ok(mut state) = self.state.lock() {
                    state.set_speaking(speaking);
                }
            },
        }
    }

    pub fn view(&self) -> iced::Element<'_, Message> {
        if let Ok(state) = self.state.lock() {
            let row =  {
                widget::row![widget::image(
                    self.images.get(state.current_image()).unwrap()
                )]
            };
            let column = widget::column![
                widget::text(
                    if state.speaking() {
                        "speaking"
                    } else {
                        ""
                    }
                ),
                row,
            ];
            widget::container(column)
                .style(|_| {
                    widget::container::Style::default()
                        .background(Background::Color(Color::TRANSPARENT))
                })
                .center_x(iced::Length::Fill)
                .center_y(iced::Length::Fill)
                .width(iced::Length::Fill)
                .height(iced::Length::Fill)
                .into()
        } else {
            widget::container(widget::text("critical error encountered, restart pretty please."))
                .center_x(iced::Length::Fill)
                .center_y(iced::Length::Fill)
                .width(iced::Length::Fill)
                .height(iced::Length::Fill)
                .into()
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
