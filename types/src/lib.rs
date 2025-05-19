pub use cli::{Args, ArgsError, ValidArgs};
pub use color::Color;
pub use config::{Config, AudioConfig};
pub use error::FrontendError;
pub use frontend::Frontend;
pub use numbers::TwoInts;

pub use clap::Parser;

mod cli;
mod color;
mod config;
mod error;
mod frontend;
mod numbers;
