macro_rules! arctex {
    ($value:expr) => {
        std::sync::Arc::new(std::sync::Mutex::new($value))
    };
}
pub(crate) use arctex;

macro_rules! interval {
    ($duration:expr, $block:block) => {
        let mut interval = tokio::time::interval($duration);
        loop {
            interval.tick().await;
            $block
        }
    };
}
pub(crate) use interval;

use std::{num::ParseIntError, str::FromStr};

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ColorParseError {
    #[error("Invalid format for color, should be \"r b g\"")]
    InvalidFormat,
    #[error("The provided number was incorrect for a color component")]
    InvalidNumber(#[from] ParseIntError),
}

#[derive(Clone, Copy)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl FromStr for Color {
    type Err = ColorParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split: Vec<&str> = s.split_whitespace().collect();
        if split.len() == 3 {
            let r = split[0].parse()?;
            let g = split[1].parse()?;
            let b = split[2].parse()?;

            Ok(Self { r, g, b })
        } else {
            Err(ColorParseError::InvalidFormat)
        }
    }
}

impl From<(u8, u8, u8)> for Color {
    fn from(value: (u8, u8, u8)) -> Self {
        Self {
            r: value.0,
            g: value.1,
            b: value.2,
        }
    }
}

impl From<Color> for iced::Color {
    fn from(value: Color) -> Self {
        iced::Color {
            r: value.r as f32 / 255.0,
            g: value.g as f32 / 255.0,
            b: value.b as f32 / 255.0,
            a: 1.0,
        }
    }
}
