use std::str::FromStr;

use crate::ArgsError;

#[derive(Clone, Copy)]
pub enum Frontend {
    #[cfg(feature = "tauri-frontend")]
    Tauri,
    #[cfg(feature = "iced-frontend")]
    Iced,
}

impl Default for Frontend {
    #[cfg(feature = "tauri-frontend")]
    fn default() -> Self {
        Frontend::Tauri
    }

    #[cfg(all(feature = "iced-frontend", not(feature = "tauri-frontend")))]
    fn default() -> Self {
        Frontend::Iced
    }
}

impl FromStr for Frontend {
    type Err = ArgsError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            #[cfg(feature = "iced-frontend")]
            "iced" | "native" => Ok(Frontend::Iced),
            #[cfg(feature = "tauri-frontend")]
            "tauri" | "web" => Ok(Frontend::Tauri),
            _ => Err(ArgsError::InvalidFrontend(String::from(s))),
        }
    }
}
