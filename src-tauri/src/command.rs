use tauri::State;

use crate::player::{track::Track, Player};

#[tauri::command]
pub fn stop(player: State<Player>) {
    player.stop();
}

#[tauri::command]
pub fn pause(player: State<Player>) {
    player.pause();
}

#[tauri::command]
pub fn play(player: State<Player>) {
    player.play();
}

#[tauri::command]
pub fn is_playing(player: State<Player>) -> bool {
    player.is_playing()
}

#[tauri::command]
pub fn get_playlist(player: State<Player>) -> Vec<Track> {
    player.get_playlist()
}
