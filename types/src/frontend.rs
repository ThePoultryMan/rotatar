use std::str::FromStr;

use crate::ArgsError;

#[derive(Clone, Copy)]
pub enum Frontend {
    Iced,
}

impl FromStr for Frontend {
    type Err = ArgsError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "iced" | "native" => Ok(Frontend::Iced),
            _ => Err(ArgsError::InvalidFrontend(String::from(s))),
        }
    }
}
