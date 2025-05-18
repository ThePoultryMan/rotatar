use std::{env, time::Duration};

use app::App;
use iced::Task;
use rotatar_backend::{Message, audio::AudioHandler};
use rotatar_types::{Config, FrontendError, TwoInts, ValidArgs};
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
    let managed_config = app.config();
    tokio::spawn(async move {
        let modifiers = if let Ok(config) = managed_config.lock() {
            config.screen_information().modifier(env::consts::OS)
        } else {
            TwoInts::new(0, 0)
        };
        let receiver = rotatar_backend::get_mouse_pos(Duration::from_millis(100), modifiers).await;
        loop {
            if let Ok(position) = receiver.recv().await {
                let mut message_to_send = None;
                match state.lock() {
                    Ok(mut state) => {
                        if state.set_current_image_xy(position.x(), position.y()) {
                            message_to_send = Some(Message::CurrentImageChanged);
                        }
                    }
                    Err(error) => todo!("{error}"),
                }
                if let Some(message_to_send) = message_to_send {
                    sender.send(message_to_send).await.unwrap();
                }
            }
        }
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
