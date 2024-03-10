use std::collections::VecDeque;

use tauri::{AppHandle, State};

use crate::player::{track::Track, Player};

#[tauri::command]
pub async fn stop(player: State<'_, Player>) -> Result<(), ()> {
    player.stop().await;
    Ok(())
}

#[tauri::command]
pub fn pause(player: State<Player>) {
    player.pause();
}

#[tauri::command]
pub async fn play(player: State<'_, Player>) -> Result<(), ()> {
    player.play().await;
    Ok(())
}

#[tauri::command]
pub async fn is_playing(player: State<'_, Player>) -> Result<bool, ()> {
    Ok(player.is_playing().await)
}

#[tauri::command]
pub async fn get_playlist(player: State<'_, Player>) -> Result<VecDeque<Track>, ()> {
    Ok(player.get_playlist().await)
}

#[tauri::command]
pub fn set_volume(player: State<Player>, volume: f32) {
    player.set_volume(volume);
}

#[tauri::command]
pub fn playtime(player: State<Player>) -> f64 {
    player.playtime().as_secs_f64()
}
