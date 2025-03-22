use std::sync::Arc;

use async_channel::{Receiver, Sender};

use crate::audio::{AudioHandler, AudioStatus};

// use crate::audio::AudioStatus;

#[derive(Debug, Clone)]
pub enum Message {
    SetupAudio(AudioHandler),
    UpdateAudioStatus(AudioStatus),
    OutsideListenerReady(Sender<Arc<Receiver<Message>>>),
    CurrentImageChanged,
    SensitivityChanged(f32),
}
