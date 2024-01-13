// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use player::Player;

mod command;
mod player;

fn main() {
    tauri::Builder::default()
        .manage(Player::default())
        .invoke_handler(tauri::generate_handler![
            command::play,
            command::stop,
            command::pause,
            command::start,
            command::is_playing
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
