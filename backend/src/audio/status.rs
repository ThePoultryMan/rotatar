use serde::Serialize;

use super::AudioHandler;

#[derive(Clone, Debug, Serialize)]
pub enum AudioStatus {
    Ready,
    Closed,
    Polling {
        #[serde(skip_serializing)]
        audio_handler: Option<AudioHandler>
    },
}

impl PartialEq for AudioStatus {
    fn eq(&self, other: &Self) -> bool {
        core::mem::discriminant(self) == core::mem::discriminant(other)
    }
}
