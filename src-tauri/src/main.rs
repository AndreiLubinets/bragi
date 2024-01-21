// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use menu::{event_handler, menu};
use player::Player;

mod command;
mod menu;
mod player;

fn main() {
    tauri::Builder::default()
        .manage(Player::default())
        .menu(menu())
        .on_menu_event(event_handler())
        .invoke_handler(tauri::generate_handler![
            command::play,
            command::stop,
            command::pause,
            command::is_playing,
            command::get_playlist
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
