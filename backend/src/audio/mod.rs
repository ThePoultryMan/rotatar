use std::time::Duration;

pub use error::AudioError;
pub use handler::AudioHandler;
pub use status::AudioStatus;

use crate::Message;

mod error;
mod handler;
mod status;

pub struct AudioHandlerResult {
    audio_handler: AudioHandler,
    error: AudioError,
}

pub enum AudioMessage {
    Stop,
}

impl AudioHandlerResult {
    pub fn result(&self) -> AudioError {
        self.error
    }

    pub fn audio_handler(self) -> AudioHandler {
        self.audio_handler
    }
}

pub async fn handle_audio(mut audio_handler: AudioHandler, device: usize) -> Message {
    if audio_handler.set_current_input_device(device) {
        let result = audio_handler.play().await;
        // AudioResult::Closed is the only time that everything is ok.
        if result.result() == AudioError::Closed
            || result.result() == AudioError::DeviceNotAvailable
        {
            let audio_handler = result.audio_handler();
            let _ = audio_handler
                .sender()
                .send(Message::UpdateAudioStatus(
                    AudioStatus::Closed,
                    Some(audio_handler.clone()),
                ))
                .await;
            Message::SetupAudio(audio_handler)
        } else {
            panic!(
                "AudioResult with unhandled error occurred:\n{}",
                result.result()
            );
        }
    } else {
        Message::UpdateAudioStatus(AudioStatus::Polling, Some(audio_handler))
    }
}

pub async fn wait_for_audio(mut audio_handler: AudioHandler, device: usize) -> Message {
    tokio::time::sleep(Duration::from_millis(75)).await;
    audio_handler.update_input_devices();
    handle_audio(audio_handler, device).await
}
