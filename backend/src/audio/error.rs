use cpal::StreamError;
use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Error)]
pub enum AudioError {
    #[error("An error occurred while listening to microphone input")]
    Play,
    #[error("AudioHolder naturally closed")]
    Closed,
    #[error("No config was found for used device.")]
    NoConfig,
    #[error("An error occurred while creating the audio input stream")]
    BuildStreamError,
    #[error("Device not available. It was most likely disconnected while/before it was being used")]
    DeviceNotAvailable,
    #[error("An error occurred while running the cpal stream")]
    StreamError,
    #[error("No device with that name was found.")]
    NoDevice,
}

impl From<StreamError> for AudioError {
    fn from(value: StreamError) -> Self {
        match value {
            StreamError::DeviceNotAvailable => Self::DeviceNotAvailable,
            StreamError::BackendSpecific { err: _ } => Self::StreamError,
        }
    }
}
