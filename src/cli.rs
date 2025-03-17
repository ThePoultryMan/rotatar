use std::path::PathBuf;

use clap::Parser;

use crate::util::Color;

#[derive(Parser)]
pub struct Args {
    #[arg(short = 'c', long = "config")]
    config_path: Option<PathBuf>,
    #[arg(long = "background")]
    background_color: Option<Color>,
}

impl Args {
    pub fn config_path(&self) -> &Option<PathBuf> {
        &self.config_path
    }

    pub fn background_color(&self) -> Option<Color> {
        self.background_color
    }
}
