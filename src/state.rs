// use crate::audio::AudioStatus;

/// Generates boiler plate for locking the mutex before setting state.
macro_rules! set_state {
    ($state:expr, $method:tt, $value:expr) => {
        if let Ok(mut state) = $state.lock() {
            state.$method($value);
        }
    };
    ($state:expr, $method:tt, $value:expr, $other:block) => {
        if let Ok(mut state) = $state.lock() {
            $other
            state.$method($value);
        }
    };
}
pub(crate) use set_state;

use crate::audio::AudioStatus;

#[derive(Clone)]
pub struct State {
    current_image: usize,
    sensitivity: f32,
    audio_status: AudioStatus,

    section_size: (i32, i32),
    x_sections: i32,
}

impl State {
    pub fn new(screen_size: (i32, i32), sections: (i32, i32)) -> Self {
        Self {
            current_image: 0,
            sensitivity: 0.0,
            audio_status: AudioStatus::Closed,
            section_size: (screen_size.0 / sections.0, screen_size.1 / sections.1),
            x_sections: sections.0,
        }
    }

    pub fn current_image(&self) -> usize {
        self.current_image
    }

    /// Attempts to set the current image, returning true if successful. Returns false when the new
    /// image is the same as the old, nothing will be set.
    pub fn set_current_image(&mut self, current_image: usize) -> bool {
        if self.current_image == current_image {
            false
        } else if current_image <= (self.section_size.0 * self.section_size.1) as usize {
            self.current_image = current_image;
            true
        } else {
            false
        }
    }

    pub fn sensitivity(&self) -> f32 {
        self.sensitivity
    }

    pub fn set_sensitivity(&mut self, sensitivity: f32) {
        self.sensitivity = sensitivity;
    }

    pub fn audio_status(&self) -> &AudioStatus {
        &self.audio_status
    }

    pub fn set_audio_status(&mut self, audio_status: AudioStatus) {
        self.audio_status = audio_status;
    }

    pub fn section_size(&self) -> (i32, i32) {
        self.section_size
    }

    pub fn x_sections(&self) -> i32 {
        self.x_sections
    }

    pub fn is_speaking(&self) -> bool {
        self.sensitivity > 0.0
    }
}

pub fn to_2d_index(x: i32, y: i32, width: i32) -> usize {
    (y * width + x) as usize
}
