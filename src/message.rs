use std::sync::Arc;

use async_channel::{Receiver, Sender};

use crate::audio::AudioStatus;

#[derive(Debug, Clone)]
pub enum Message {
    Ready(Sender<Arc<Receiver<Message>>>),
    AudioStatus(AudioStatus),
    CurrentImageChanged,
    SensitivityChanged(f32),
}
