use std::{
    collections::VecDeque,
    sync::atomic::{AtomicUsize, Ordering},
    usize,
};

use log::info;
use tauri::async_runtime::Mutex;

use super::track::Track;

pub struct Queue {
    tracks: Mutex<VecDeque<Track>>,
    current: AtomicUsize,
}

impl Queue {
    pub fn new() -> Self {
        Self {
            tracks: Mutex::new(VecDeque::new()),
            current: AtomicUsize::new(0),
        }
    }

    pub async fn add(&self, track: Track) {
        info!("Adding track to queue: {:?}", track.path());
        self.tracks.lock().await.push_back(track);
    }

    pub async fn next(&self) -> Option<Track> {
        let guard = self.tracks.lock().await;
        if guard.is_empty() || self.current.load(Ordering::Relaxed) >= guard.len() {
            return None;
        }

        let track = guard.get(self.current.fetch_add(1, Ordering::Relaxed));

        track.cloned()
    }

    /*
    pub fn current(&self) -> Option<&Track> {
        self.tracks(self.current.load(Ordering::Relaxed))
    }*/

    //TODO: Remove clone
    pub async fn get_playlist(&self) -> VecDeque<Track> {
        self.tracks.lock().await.clone()
    }
}
