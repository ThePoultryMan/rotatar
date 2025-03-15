use std::sync::{Arc, Mutex};

use iced::widget;

use crate::{message::Message, state::State};

pub struct App {
    images: Vec<String>,
    screen_size: (i32, i32),
    sections: (i32, i32),
    state: Arc<Mutex<State>>,
}

impl App {
    pub fn new() -> Self {
        let screen_size = (1920, 1080);
        let sections = (3, 3);
        Self {
            images: Vec::new(),
            screen_size,
            sections,
            state: Arc::new(Mutex::new(State::new(screen_size, sections))),
        }
    }

    pub fn update(&mut self, _: Message) -> iced::Task<Message> {
        iced::Task::none()
    }

    pub fn view(&self) -> iced::Element<'_, Message> {
        let row = if let Ok(state) = self.state.lock() {
            widget::row![widget::image(
                self.images.get(state.current_image()).unwrap()
            )]
        } else {
            widget::row![]
        };
        widget::container(row)
            .center_x(iced::Length::Fill)
            .center_y(iced::Length::Fill)
            .width(iced::Length::Fill)
            .height(iced::Length::Fill)
            .into()
    }

    /// Gets a clone of this app's [Arc](std::sync::Arc)<[Mutex](std::sync::Mutex)<[State](crate::state::State)>>
    pub fn state(&self) -> Arc<Mutex<State>> {
        self.state.clone()
    }
}
