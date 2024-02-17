use std::sync::atomic::AtomicUsize;

use tauri::async_runtime::RwLock;

use super::track::Track;
/*

#[derive(Default)]
pub struct Queue {
    playlist: RwLock<Vec<Track>>,
    current: AtomicUsize,
}

impl Queue {
    pub async fn add_track(&self, track: Track) {
        Timer
        self.playlist.write().await.push(track)
    }

}*/
