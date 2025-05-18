use std::{env, sync::Mutex, time::Duration};

use async_channel::Sender;
use rotatar_backend::{
    Message, State,
    audio::{self, AudioHandler, AudioStatus},
    get_mouse_pos, set_state,
};
use rotatar_types::Config;
use tauri::{AppHandle, Emitter, Manager, generate_context};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run(config: Config) {
    let (sender, receiver) = async_channel::unbounded();
    let build_sender = sender.clone();
    let audio_sender = sender.clone();

    tokio::spawn(async move {
        let audio_handler = AudioHandler::new(audio_sender.clone());
        audio_sender
            .send(audio::handle_audio(audio_handler).await)
            .await
            .unwrap();
    });

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(move |app| {
            app.manage(Mutex::new(State::new(
                config.screen_information().size(),
                config.sections(),
            )));
            app.manage(config);

            let app_handle_audio = app.handle().clone();
            build_sender
                .send_blocking(Message::SetupAudio(AudioHandler::new(build_sender.clone())))
                .unwrap();
            tokio::spawn(async move {
                loop {
                    if let Ok(message) = receiver.recv_blocking() {
                        handle_message(build_sender.clone(), app_handle_audio.clone(), message)
                            .await;
                    }
                }
            });
            let app_handle = app.handle().clone();
            tokio::spawn(async move {
                let state = app_handle.state::<Mutex<State>>();
                if let Ok(state) = state.lock() {
                    let _ = app_handle.emit(
                        "current-image",
                        app_handle
                            .state::<Config>()
                            .idle_images()
                            .get(state.current_image())
                            .unwrap(),
                    );
                }
                let mos_pos_receiver = get_mouse_pos(
                    Duration::from_millis(100),
                    app_handle
                        .state::<Config>()
                        .screen_information()
                        .modifier(env::consts::OS),
                )
                .await;
                loop {
                    let mut message = None;
                    if let Ok(position) = mos_pos_receiver.recv().await {
                        if let Ok(mut state) = app_handle.state::<Mutex<State>>().lock() {
                            if state.set_current_image_xy(position.x(), position.y()) {
                                message = Some(Message::CurrentImageChanged);
                            }
                        }
                    }
                    if let Some(message) = message {
                        sender.send(message).await.unwrap();
                    }
                }
            });

            Ok(())
        })
        .run(generate_context!())
        .expect("failed to run tauri app");
}

async fn handle_message(sender: Sender<Message>, app_handle: AppHandle, message: Message) {
    match message {
        Message::SetupAudio(audio_handler) => {
            handle_audio_wrapper(sender, audio_handler);
        }
        Message::UpdateAudioStatus(audio_status) => {
            if let AudioStatus::Polling { audio_handler } = audio_status {
                if let Some(audio_handler) = audio_handler {
                    wait_for_audio_wrapper(sender, audio_handler).await;
                }
            } else {
                set_state!(
                    app_handle.state::<Mutex<State>>(),
                    set_audio_status,
                    audio_status
                );
            }
        }
        Message::SensitivityChanged(sensitivity) => {
            app_handle.emit("sensitivity-changed", sensitivity).unwrap();
        }
        Message::CurrentImageChanged => {
            if let Ok(state) = app_handle.state::<Mutex<State>>().lock() {
                let _ = app_handle.emit(
                    "current-image",
                    app_handle
                        .state::<Config>()
                        .idle_images()
                        .get(state.current_image())
                        .unwrap(),
                );
            }
        }
        _ => {}
    }
}

fn handle_audio_wrapper(sender: Sender<Message>, mut audio_handler: AudioHandler) {
    tokio::spawn(async move {
        audio_handler.update_input_devices();
        let message = audio::handle_audio(audio_handler).await;
        sender.send(message).await.unwrap();
    });
}

async fn wait_for_audio_wrapper(sender: Sender<Message>, audio_handler: AudioHandler) {
    tokio::spawn(async move {
        sender
            .send(audio::wait_for_audio(audio_handler).await)
            .await
            .unwrap();
    });
}
