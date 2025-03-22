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

impl Color {
    pub fn r(&self) -> u8 {
        self.r
    }

    pub fn g(&self) -> u8 {
        self.g
    }

    pub fn b(&self) -> u8 {
        self.b
    }
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
