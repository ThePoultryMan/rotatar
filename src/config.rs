use std::path::{Path, PathBuf};

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    screen_size: (i32, i32),
    sections: (i32, i32),
    idle_images: Vec<PathBuf>,
    speaking_images: Vec<PathBuf>,
}

impl Config {
    pub fn screen_size(&self) -> (i32, i32) {
        self.screen_size
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
}
