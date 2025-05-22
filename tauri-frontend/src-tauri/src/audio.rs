use std::sync::Mutex;

use async_channel::{Receiver, Sender};
use rotatar_backend::{
    Message, State,
    audio::{AudioHandler, AudioMessage},
};
use rotatar_types::Config;
use tauri::{AppHandle, Manager};

#[tauri::command(async)]
pub async fn set_audio_device(device: String, app_handle: AppHandle) {
    let (audio_sender, audio_receiver, message_sender) =
        if let Ok(state) = app_handle.state::<Mutex<State>>().lock() {
            (
                state.audio_handler_sender(),
                state.audio_receiver(),
                state.message_sender(),
            )
        } else {
            panic!("State mutex was poisoned, found in {}", file!());
        };
    audio_sender.send(AudioMessage::Stop).await.unwrap();
    let mut audio_handler = AudioHandler::new(
        message_sender.clone(),
        audio_receiver,
        app_handle.state::<Config>().audio(),
    );
    audio_handler.set_input_device_from_name(device).unwrap();
    set_up_audio_inner(audio_handler, message_sender);
}

pub fn set_up_audio(
    sender: Sender<Message>,
    receiver: Receiver<AudioMessage>,
    app_handle: AppHandle,
) {
    set_up_audio_inner(
        AudioHandler::new(
            sender.clone(),
            receiver,
            app_handle.state::<Config>().audio(),
        ),
        sender,
    );
}

fn set_up_audio_inner(audio_handler: AudioHandler, sender: Sender<Message>) {
    tauri::async_runtime::spawn(async move {
        sender
            .send(Message::SetupAudio(audio_handler))
            .await
            .unwrap();
    });
}
