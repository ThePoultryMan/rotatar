use better_default::Default;
use serde::{Deserialize, Serialize};
use serde_inline_default::serde_inline_default;

#[serde_inline_default]
#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize)]
pub struct AudioConfig {
    #[default(6)]
    #[serde_inline_default(6)]
    magnitude_threshold: i32,
    #[default(50)]
    #[serde_inline_default(50)]
    max_magnitude: i32,
}

impl AudioConfig {
    pub fn magnitude_threshold(&self) -> i32 {
        self.magnitude_threshold
    }

    pub fn max_magnitude(&self) -> i32 {
        self.max_magnitude
    }
}
