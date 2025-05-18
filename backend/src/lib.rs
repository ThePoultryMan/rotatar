use std::time::Duration;
use std::{fs::read_to_string, io};

use async_channel::Receiver;
use mouce::{Mouse, MouseActions};
use rotatar_types::{Config, TwoInts};
use rotatar_types::{FrontendError, ValidArgs};
use thiserror::Error;

pub use message::Message;
pub use state::State;

pub mod audio;
mod message;
mod state;
mod util;

#[derive(Debug, Error)]
pub enum Error {
    #[error("No configuration file was specified")]
    NoConfig,
    #[error("Configuration file is invalid: `{0}`")]
    InvalidConfig(String),
    // #[error("An error occurred while attempting to use audio")]
    // Audio(#[from] AudioError),
    #[error("I/O Error")]
    IO(#[from] io::Error),
    #[error("Could not parse config file")]
    Parse(#[from] serde_json::Error),
    #[error("Error from gui frontend")]
    Backend(#[from] FrontendError),
}

pub async fn run(args: &ValidArgs) -> Result<Config, Error> {
    let config: Config = serde_json::from_str(&read_to_string(args.config_path())?)?;
    if config.image_count() < config.total_sections() {
        return Err(Error::InvalidConfig(format!(
            "You cannot have less images then you have sections. You only have {} images while you have {} sections",
            config.image_count(),
            config.total_sections()
        )));
    }

    Ok(config)
}

pub async fn get_mouse_pos(interval: Duration, modifiers: TwoInts) -> Receiver<TwoInts> {
    let (sender, out_receiver) = async_channel::unbounded();

    tokio::spawn(async move {
        let mouse = Mouse::new();
        interval!(interval, {
            if let Ok(position) = mouse.get_position() {
                sender
                    .send(TwoInts::from(position) + modifiers)
                    .await
                    .unwrap();
            }
        });
    });
    out_receiver
}
