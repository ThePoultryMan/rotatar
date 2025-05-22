use std::{env, sync::Mutex, time::Duration};

use async_channel::Sender;
use audio::set_up_audio;
use rotatar_backend::{Message, State, audio::AudioStatus, get_mouse_pos, set_state};
use rotatar_types::Config;
use tauri::{AppHandle, Emitter, Manager, generate_context};

mod audio;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run(config: Config) {
    let (sender, receiver) = async_channel::unbounded();
    let (audio_sender, audio_receiver) = async_channel::bounded(5);

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            get_config,
            get_state,
            audio::set_audio_device
        ])
        .setup(move |app| {
            app.manage(Mutex::new(State::new(
                sender.clone(),
                config.screen_information().size(),
                config.sections(),
                audio_sender,
                audio_receiver.clone(),
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

            set_up_audio(sender.clone(), audio_receiver, app.handle().clone());

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
            let audio_device = app_handle.state::<Config>().audio().current_device();
            tauri::async_runtime::spawn(async move {
                sender
                    .send(rotatar_backend::audio::handle_audio(audio_handler, audio_device).await)
                    .await
                    .unwrap();
            });
        }
        Message::UpdateAudioStatus(audio_status, audio_handler) => {
            if audio_status == AudioStatus::Polling {
                if let Some(audio_handler) = audio_handler {
                    let audio_device = app_handle.state::<Config>().audio().current_device();
                    tauri::async_runtime::spawn(async move {
                        rotatar_backend::audio::wait_for_audio(audio_handler, audio_device).await;
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
                    .unwrap_or_else(|_| panic!("State mutex was poisoned in {}", file!()))
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
        .unwrap_or_else(|_| panic!("The state mutex was poisoned. Found in: {}", file!()))
        .clone()
}
