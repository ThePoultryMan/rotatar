use std::path::PathBuf;

use clap::Parser;

#[derive(Parser)]
pub struct Args {
    #[arg(short = 'c', long = "config")]
    config_path: Option<PathBuf>,
}

impl Args {
    pub fn config_path(&self) -> &Option<PathBuf> {
        &self.config_path
    }
}
