use std::time::Duration;

use app::App;
use iced::Task;
use rotatar_backend::{AudioHandler, Message, to_2d_index};
use rotatar_types::{Config, FrontendError, ValidArgs};
use util::ToIcedColor;

mod app;
mod util;

pub async fn run(args: ValidArgs, config: Config) -> Result<(), FrontendError> {
    let (sender, receiver) = async_channel::unbounded();
    let cloned_sender = sender.clone();
    let background_color = if let Some(background_color) = args.background_color() {
        background_color.into_iced()
    } else {
        iced::Color::TRANSPARENT
    };
    let app = App::new(config, background_color, receiver);

    let state = app.state();
    tokio::spawn(async move {
        rotatar_backend::get_mouse_pos(Duration::from_millis(100), async move |mouse_position| {
            let mut message_to_send = None;
            match state.lock() {
                Ok(mut state) => {
                    let (section_size, x_sections) = (state.section_size(), state.x_sections());
                    let new_image = to_2d_index(
                        mouse_position.0 / section_size.0,
                        mouse_position.1 / section_size.1,
                        x_sections,
                    );
                    if state.current_image() != new_image {
                        state.set_current_image(new_image);
                        message_to_send = Some(Message::CurrentImageChanged);
                    }
                }
                Err(error) => {
                    todo!("{error}")
                }
            }
            if let Some(message_to_send) = message_to_send {
                let _ = sender.send(message_to_send).await;
            }
        })
        .await;
    });

    let _ = cloned_sender
        .send(Message::SetupAudio(AudioHandler::new(
            cloned_sender.clone(),
        )))
        .await;
    let result = iced::application("rotatar", App::update, App::view)
        .transparent(true)
        .subscription(App::subscription)
        .run_with(|| (app, Task::none()));

    if result.is_err() {
        Err(FrontendError::Iced)
    } else {
        Ok(())
    }
}
