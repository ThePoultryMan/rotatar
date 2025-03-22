use super::AudioHandler;

#[derive(Debug, Clone)]
pub enum AudioStatus {
    Ready,
    Closed,
    Polling { audio_handler: Option<AudioHandler> },
}

impl PartialEq for AudioStatus {
    fn eq(&self, other: &Self) -> bool {
        core::mem::discriminant(self) == core::mem::discriminant(other)
    }
}
