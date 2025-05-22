/// Generates boiler plate for locking the mutex before setting state.
#[macro_export]
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

use async_channel::{Receiver, Sender};
use rotatar_types::TwoInts;
use serde::Serialize;

use crate::{
    Message,
    audio::{AudioMessage, AudioStatus},
};

#[derive(Clone, Serialize)]
pub struct State {
    current_image: usize,
    sensitivity: f32,
    #[serde(skip_serializing)]
    message_sender: Sender<Message>,

    audio_status: AudioStatus,
    #[serde(skip_serializing)]
    audio_sender: Sender<AudioMessage>,
    #[serde(skip_serializing)]
    audio_receiver: Receiver<AudioMessage>,
    audio_devices: Vec<String>,

    section_size: (i32, i32),
    x_sections: i32,
}

impl State {
    pub fn new(
        message_sender: Sender<Message>,
        screen_size: TwoInts,
        sections: (i32, i32),
        audio_sender: Sender<AudioMessage>,
        audio_receiver: Receiver<AudioMessage>,
    ) -> Self {
        let mut state = Self {
            current_image: 0,
            sensitivity: 0.0,
            message_sender,
            audio_status: AudioStatus::Closed,
            audio_sender,
            audio_receiver,
            audio_devices: Vec::new(),
            section_size: (screen_size.x() / sections.0, screen_size.y() / sections.1),
            x_sections: sections.0,
        };
        state.set_current_image_xy(state.section_size().0 / 2, state.section_size().1 / 2);
        state
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

    pub fn set_current_image_xy(&mut self, x: i32, y: i32) -> bool {
        let new_image = to_2d_index(
            x / self.section_size.0,
            y / self.section_size.1,
            self.x_sections,
        );
        self.set_current_image(new_image)
    }

    pub fn sensitivity(&self) -> f32 {
        self.sensitivity
    }

    pub fn set_sensitivity(&mut self, sensitivity: f32) {
        self.sensitivity = sensitivity;
    }

    pub fn message_sender(&self) -> Sender<Message> {
        self.message_sender.clone()
    }

    pub fn audio_status(&self) -> &AudioStatus {
        &self.audio_status
    }

    pub fn set_audio_status(&mut self, audio_status: AudioStatus) {
        self.audio_status = audio_status;
    }

    pub fn audio_handler_sender(&self) -> Sender<AudioMessage> {
        self.audio_sender.clone()
    }

    pub fn audio_receiver(&self) -> Receiver<AudioMessage> {
        self.audio_receiver.clone()
    }

    pub fn set_audio_devices(&mut self, audio_devices: Vec<String>) {
        self.audio_devices = audio_devices;
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

fn to_2d_index(x: i32, y: i32, width: i32) -> usize {
    (y * width + x) as usize
}
