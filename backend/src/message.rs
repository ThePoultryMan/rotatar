use std::sync::Arc;

use async_channel::{Receiver, Sender};

use crate::audio::{AudioHandler, AudioStatus};

#[derive(Debug, Clone)]
pub enum Message {
    SetupAudio(AudioHandler),
    UpdateAudioStatus(AudioStatus),
    OutsideListenerReady(Sender<Arc<Receiver<Message>>>),
    CurrentImageChanged,
    SensitivityChanged(f32),
    MagnitudeChanged(u32),
}
