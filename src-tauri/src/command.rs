use std::path::PathBuf;

use tauri::State;

use crate::player::Player;

#[tauri::command]
pub async fn play(player: State<'_, Player>, path: PathBuf) -> Result<(), ()> {
    player.play(path).await;

    Ok(())
}

#[tauri::command]
pub fn stop(player: State<Player>) {
    player.stop();
}

#[tauri::command]
pub fn pause(player: State<Player>) {
    player.pause();
}

#[tauri::command]
pub fn start(player: State<Player>) {
    player.start();
}

#[tauri::command]
pub fn is_playing(player: State<Player>) -> bool {
    player.is_playing()
}
