use std::{collections::VecDeque, error::Error, path::Path};

use tauri::{Manager, Runtime, State};

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
    Ok(player.is_playing())
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

#[tauri::command]
pub async fn play_queue<R: Runtime>(
    app: tauri::AppHandle<R>,
    path: impl AsRef<Path>,
) -> Result<(), Box<dyn Error>> {
    let player = app.state::<Player>();
    player.open(path).await?;
    app.emit_all("open", ())?;
    if !player.is_playing() {
        player.play_queue().await?;
    }

    Ok(())
}
