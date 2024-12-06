use std::{collections::VecDeque, error::Error, path::PathBuf, time::Duration};

use log::{error, warn};
use tauri::{Emitter, Manager, Runtime, State};

use crate::player::{track::AlbumCover, track::Track, Player};

#[tauri::command]
pub async fn stop(player: State<'_, Player>) -> Result<(), String> {
    player.stop().await;
    Ok(())
}

#[tauri::command]
pub fn pause(player: State<Player>) {
    player.pause();
}

#[tauri::command]
pub async fn play(player: State<'_, Player>) -> Result<(), String> {
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
pub fn adjust_volume(player: State<Player>, step: f32) {
    player.adjust_volume(step);
}

#[tauri::command]
pub fn get_volume(player: State<Player>) -> f32 {
    player.volume()
}

#[tauri::command]
pub fn playtime(player: State<Player>) -> f64 {
    player.playtime().as_secs_f64()
}

#[tauri::command]
pub async fn play_queue<R: Runtime>(
    app: &tauri::AppHandle<R>,
    paths: Vec<PathBuf>,
) -> Result<(), Box<dyn Error>> {
    let player = app.state::<Player>();

    for path in paths {
        player.open(path).await?;
    }

    app.emit("open", ())?;
    if !player.is_playing() {
        player.play_queue().await?;
    }

    Ok(())
}

#[tauri::command]
pub async fn change_track(player: State<'_, Player>, index: usize) -> Result<(), ()> {
    player.change_track(index).await.map_err(|_| ())
}

#[tauri::command]
pub async fn get_album_cover(player: State<'_, Player>) -> Result<AlbumCover, ()> {
    player
        .get_album_cover()
        .await
        .inspect_err(|err| warn!("{}", err))
        .map_err(|_| ())
}

#[tauri::command]
pub async fn next_track(player: State<'_, Player>) -> Result<(), String> {
    player.next().await;

    Ok(())
}

#[tauri::command]
pub async fn previous_track(player: State<'_, Player>) -> Result<(), String> {
    player.previous().await;

    Ok(())
}

#[tauri::command]
pub fn seek(player: State<Player>, pos: f64) -> Result<(), ()> {
    player
        .seek(Duration::from_secs_f64(pos))
        .inspect_err(|err| error!("{}", err))
        .map_err(|_| ())
}
