pub use error::AudioResult;
pub use handler::AudioHandler;
pub use status::AudioStatus;

mod error;
mod handler;
mod status;

pub struct AudioHandlerResult {
    audio_handler: AudioHandler,
    result: AudioResult,
}

impl AudioHandlerResult {
    pub fn result(&self) -> AudioResult {
        self.result
    }

    pub fn audio_handler(self) -> AudioHandler {
        self.audio_handler
    }
}
