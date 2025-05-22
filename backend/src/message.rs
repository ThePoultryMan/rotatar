use std::sync::Arc;

use async_channel::{Receiver, Sender};
use rotatar_types::Config;

use crate::audio::{AudioHandler, AudioStatus};

#[derive(Debug, Clone)]
pub enum Message {
    SetupAudio(AudioHandler),
    UpdateAudioStatus(AudioStatus, Option<AudioHandler>),
    OutsideListenerReady(Sender<Arc<Receiver<Message>>>),
    CurrentImageChanged,
    SensitivityChanged(f32),
    MagnitudeChanged(i32),
    ConfigChanged(Config),
    AudioDevicesChanged(Vec<String>),
}
