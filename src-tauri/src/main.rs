// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use log::error;
use menu::{event_handler, menu};
use player::{Event, Player};
use tauri::{async_runtime, Emitter, Manager};

mod command;
mod menu;
mod player;
mod util;

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(
            tauri_plugin_log::Builder::default()
                .level(log::LevelFilter::Info)
                .build(),
        )
        .setup(|app| {
            let (player, rx) = Player::new().expect("failed to init player");
            let handle = app.handle().clone();
            app.manage(player);

            async_runtime::spawn(async move {
                while let Ok(event) = rx.recv() {
                    match event {
                        Event::TrackChanged(index) => {
                            if let Err(err) = handle.emit("track_changed", index) {
                                error!("{}", err);
                            }
                        }
                        Event::PlaybackStopped => {
                            if let Err(err) = handle.emit("playback_stopped", ()) {
                                error!("{}", err);
                            }
                        }
                        Event::VolumeUpdated => {
                            if let Err(err) = handle.emit("volume_updated", ()) {
                                error!("{}", err);
                            }
                        }
                    }
                }
            });

            Ok(())
        })
        .menu(menu())
        .on_menu_event(event_handler())
        .invoke_handler(tauri::generate_handler![
            command::play,
            command::stop,
            command::pause,
            command::is_playing,
            command::get_playlist,
            command::set_volume,
            command::adjust_volume,
            command::get_volume,
            command::playtime,
            command::change_track,
            command::get_album_cover,
            command::next_track,
            command::previous_track,
            command::seek,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
