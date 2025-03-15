pub struct State {
    current_image: usize,
    section_size: (i32, i32),
}

impl State {
    pub fn new(screen_size: (i32, i32), sections: (i32, i32)) -> Self {
        Self {
            current_image: 0,
            section_size: (screen_size.0 / sections.0, screen_size.1 / sections.1),
        }
    }

    pub fn current_image(&self) -> usize {
        self.current_image
    }
    
    pub fn set_current_image(&mut self, current_image: usize) {
        self.current_image = current_image;
    }

    pub fn section_size(&self) -> (i32, i32) {
        self.section_size
    }
}

pub fn to_2d_index(x: i32, y: i32, width: i32) -> usize {
    (y * width + x) as usize
}
