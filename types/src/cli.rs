use std::path::PathBuf;

use clap::Parser;
use thiserror::Error;

use crate::{color::Color, frontend::Frontend};

#[derive(Debug, Error)]
pub enum ArgsError {
    #[error("No configuration file was passed to the app")]
    NoConfig,
    #[error("An invalid frontend was specified because `{0}` is not a frontend")]
    InvalidFrontend(String),
    #[error("Some part of the provided args was invalid and could not be fixed")]
    Invalid,
}

#[derive(Parser)]
pub struct Args {
    #[arg(short = 'c', long = "config")]
    config_path: Option<PathBuf>,
    #[arg(short = 'f', long = "frontend")]
    frontend: Option<Frontend>,
    #[arg(long = "background")]
    background_color: Option<Color>,
}

pub struct ValidArgs {
    config_path: PathBuf,
    frontend: Frontend,
    background_color: Option<Color>,
}

impl ValidArgs {
    pub fn config_path(&self) -> &PathBuf {
        &self.config_path
    }

    pub fn frontend(&self) -> Frontend {
        self.frontend
    }

    pub fn background_color(&self) -> Option<Color> {
        self.background_color
    }
}

impl TryFrom<Args> for ValidArgs {
    type Error = ArgsError;

    fn try_from(value: Args) -> Result<Self, Self::Error> {
        if let Some(config_path) = value.config_path {
            Ok(ValidArgs {
                config_path,
                frontend: value.frontend.unwrap_or(Frontend::Tauri),
                background_color: value.background_color,
            })
        } else {
            Err(ArgsError::Invalid)
        }
    }
}
