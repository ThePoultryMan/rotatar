use std::{fs::read_to_string, io, thread, time::Duration};

use app::App;
use clap::Parser;
use cli::Args;
use config::Config;
use iced::{
    task::Handle,
    widget::{self, Image},
    Task,
};
use mouce::{Mouse, MouseActions};
use state::to_2d_index;
use thiserror::Error;
use tokio::time;

mod app;
mod cli;
mod config;
mod message;
mod state;

#[derive(Debug, Error)]
enum Error {
    #[error("No configuration file was specified")]
    NoConfig,
    #[error("Configuration file is invalid: `{0}")]
    InvalidConfig(String),
    #[error("I/O Error")]
    IO(#[from] io::Error),
    #[error("Could not parse config file")]
    ParseError(#[from] serde_json::Error),
    #[error("Error from iced gui")]
    Iced(#[from] iced::Error),
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args = Args::parse();

    if let Some(config_path) = args.config_path() {
        let config: Config = serde_json::from_str(&read_to_string(config_path)?)?;
        if config.image_count() < config.total_sections() {
            return Err(
                Error::InvalidConfig(
                    format!(
                        "You cannot have less images then you have sections. You only have {} images while you have {} sections",
                        config.image_count(),
                        config.total_sections()
                    )
                )
            );
        }

        let app = App::new();

        let state = app.state();
        tokio::spawn(async move {
            let mouse = Mouse::new();
            let mut interval = time::interval(Duration::from_millis(100));
            loop {
                interval.tick().await;
                if let Ok(mouse_position) = mouse.get_position() {
                    match state.lock() {
                        Ok(mut state) => {
                            let section_size = state.section_size();
                            state.set_current_image(to_2d_index(
                                mouse_position.0 / section_size.0,
                                mouse_position.1 / section_size.1,
                                section_size.0,
                            ));
                        }
                        Err(error) => {
                            todo!("{error}")
                        }
                    }
                }
            }
        });

        iced::application("rotatar", App::update, App::view).run_with(|| (app, Task::none()))?;
    } else {
        return Err(Error::NoConfig);
    }

    Ok(())
}
