// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use menu::{event_handler, menu};
use player::Player;

mod command;
mod log;
mod menu;
mod player;

fn main() {
    log::init_logger().expect("failed to init logger");

    tauri::Builder::default()
        .manage(Player::new().expect("failed to init player"))
        .menu(menu())
        .on_menu_event(event_handler())
        .invoke_handler(tauri::generate_handler![
            command::play,
            command::stop,
            command::pause,
            command::is_playing,
            command::get_playlist,
            command::set_volume,
            command::playtime
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
