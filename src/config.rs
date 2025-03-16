use std::path::PathBuf;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    screen_size: (i32, i32),
    sections: (i32, i32),
    images: Vec<PathBuf>,
}

impl Config {
    pub fn screen_size(&self) -> (i32, i32) {
        self.screen_size
    }

    pub fn sections(&self) -> (i32, i32) {
        self.sections
    }

    pub fn images(&self) -> Vec<PathBuf> {
        self.images.clone()
    }

    pub fn total_sections(&self) -> usize {
        (self.sections.0 * self.sections.1) as usize
    }

    pub fn image_count(&self) -> usize {
        self.images.len()
    }
}
