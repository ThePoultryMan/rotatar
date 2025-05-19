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

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![get_config, get_state])
        .setup(move |app| {
            app.manage(Mutex::new(State::new(
                config.screen_information().size(),
                config.sections(),
            )));
            app.manage(config);

            let mouse_app_handle = app.handle().clone();
            let mouse_sender = sender.clone();
            tauri::async_runtime::spawn(async move {
                let mouse_pos_receiver = get_mouse_pos(
                    Duration::from_millis(100),
                    mouse_app_handle
                        .state::<Config>()
                        .screen_information()
                        .modifier(env::consts::OS),
                )
                .await;
                loop {
                    if let Ok(position) = mouse_pos_receiver.recv().await {
                        let mut message_to_send = None;
                        match mouse_app_handle.state::<Mutex<State>>().lock() {
                            Ok(mut state) => {
                                if state.set_current_image_xy(position.x(), position.y()) {
                                    message_to_send = Some(Message::CurrentImageChanged);
                                }
                            }
                            Err(error) => todo!("{error}"),
                        }
                        if let Some(message_to_send) = message_to_send {
                            mouse_sender.send(message_to_send).await.unwrap();
                        }
                    }
                }
            });

            let audio_app_handle = app.handle().clone();
            let audio_sender = sender.clone();
            tauri::async_runtime::spawn(async move {
                audio_sender
                    .send(Message::SetupAudio(AudioHandler::new(
                        audio_sender.clone(),
                        audio_app_handle.state::<Config>().audio(),
                    )))
                    .await
                    .unwrap();
            });

            let message_app_handle = app.handle().clone();
            tokio::spawn(async move {
                let message_sender = sender.clone();
                loop {
                    if let Ok(message) = receiver.recv_blocking() {
                        handle_message(message_sender.clone(), message_app_handle.clone(), message)
                            .await;
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
        Message::SetupAudio(mut audio_handler) => {
            audio_handler.update_input_devices();
            tauri::async_runtime::spawn(async move {
                sender
                    .send(audio::handle_audio(audio_handler).await)
                    .await
                    .unwrap();
            });
        }
        Message::UpdateAudioStatus(audio_status) => {
            if let AudioStatus::Polling { audio_handler } = audio_status {
                if let Some(audio_handler) = audio_handler {
                    tauri::async_runtime::spawn(async move {
                        audio::wait_for_audio(audio_handler).await;
                    });
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
            let _ = app_handle.emit(
                "current-image-changed",
                app_handle
                    .state::<Mutex<State>>()
                    .lock()
                    .expect(&format!("State mutex was poisoned in {}", file!()))
                    .current_image(),
            );
        }
        Message::MagnitudeChanged(magnitude) => {
            app_handle.emit("magnitude-changed", magnitude).unwrap();
        }
        Message::ConfigChanged(config) => {
            app_handle.emit("config-changed", config).unwrap();
        }
        Message::AudioDevicesChanged(devices) => {
            app_handle.emit("audio-devices-changed", &devices).unwrap();
            set_state!(
                app_handle.state::<Mutex<State>>(),
                set_audio_devices,
                devices
            );
        }
        _ => {}
    }
}

#[tauri::command]
fn get_config(app_handle: AppHandle) -> Config {
    app_handle.state::<Config>().inner().clone()
}

#[tauri::command]
fn get_state(app_handle: AppHandle) -> State {
    app_handle
        .state::<Mutex<State>>()
        .lock()
        .expect(&format!(
            "The state mutex was poisoned. Found in: {}",
            file!()
        ))
        .clone()
}
