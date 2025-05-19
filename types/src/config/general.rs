use std::{collections::HashMap, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::TwoInts;

use super::audio::AudioConfig;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    audio: AudioConfig,
    sections: (i32, i32),
    idle_images: Vec<PathBuf>,
    speaking_images: Vec<PathBuf>,
    screen_information: ScreenInformation,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ScreenInformation {
    size: TwoInts,
    modifiers: HashMap<String, TwoInts>,
}

impl Config {
    pub fn audio(&self) -> AudioConfig {
        self.audio
    }

    pub fn sections(&self) -> (i32, i32) {
        self.sections
    }

    pub fn idle_images(&self) -> &Vec<PathBuf> {
        &self.idle_images
    }

    pub fn speaking_images(&self) -> &Vec<PathBuf> {
        &self.speaking_images
    }

    pub fn total_sections(&self) -> usize {
        (self.sections.0 * self.sections.1) as usize
    }

    pub fn image_count(&self) -> usize {
        self.idle_images.len()
    }

    pub fn screen_information(&self) -> &ScreenInformation {
        &self.screen_information
    }
}

impl ScreenInformation {
    pub fn size(&self) -> TwoInts {
        self.size
    }

    pub fn modifier(&self, os: &str) -> TwoInts {
        *self.modifiers.get(os).unwrap_or(&TwoInts::default())
    }
}
