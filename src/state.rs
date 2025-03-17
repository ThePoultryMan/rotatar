#[derive(Clone, Copy)]
pub struct State {
    current_image: usize,
    speaking: bool,

    section_size: (i32, i32),
    x_sections: i32,
}

impl State {
    pub fn new(screen_size: (i32, i32), sections: (i32, i32)) -> Self {
        Self {
            current_image: 0,
            speaking: false,
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

    pub fn speaking(&self) -> bool {
        self.speaking
    }

    pub fn set_speaking(&mut self, speaking: bool) {
        self.speaking = speaking;
    }

    pub fn section_size(&self) -> (i32, i32) {
        self.section_size
    }

    pub fn x_sections(&self) -> i32 {
        self.x_sections
    }
}

pub fn to_2d_index(x: i32, y: i32, width: i32) -> usize {
    (y * width + x) as usize
}
