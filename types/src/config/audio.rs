use better_default::Default;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Default, Serialize, Deserialize)]
pub struct AudioConfig {
    #[default(6)]
    #[serde(default = "default_magnitude_threshold")]
    magnitude_threshold: i32,
}

impl AudioConfig {
    pub fn magnitude_threshold(&self) -> i32 {
        self.magnitude_threshold
    }
}

const fn default_magnitude_threshold() -> i32 {
    6
}
