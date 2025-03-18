use std::sync::Arc;

use async_channel::{Receiver, Sender};

#[derive(Debug, Clone)]
pub enum Message {
    Ready(Sender<Arc<Receiver<Message>>>),
    HasAudioInput(bool),
    CurrentImageChanged,
    SensitivityChanged(f32),
}
