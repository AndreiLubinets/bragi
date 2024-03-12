use std::{
    collections::VecDeque,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    usize,
};

use log::info;
use tauri::async_runtime::Mutex;

use super::track::Track;

pub struct Queue {
    tracks: Arc<Mutex<VecDeque<Track>>>,
    current: AtomicUsize,
}

impl Queue {
    pub fn new() -> Self {
        Self {
            tracks: Arc::new(Mutex::new(VecDeque::new())),
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

    pub fn current(&self) -> usize {
        self.current.load(Ordering::Relaxed)
    }

    //TODO: Remove clone
    pub async fn get_playlist(&self) -> VecDeque<Track> {
        self.tracks.lock().await.clone()
    }
}

#[cfg(test)]
mod tests {
    use crate::player::track::Track;
    use std::collections::VecDeque;

    use super::Queue;

    #[tokio::test]
    async fn add() {
        let queue = Queue::new();
        queue.add(Track::default()).await;

        assert_eq!(Track::default(), queue.next().await.unwrap());
    }

    #[tokio::test]
    async fn next() {
        let queue = Queue::new();
        queue.add(Track::default()).await;
        queue.add(Track::default()).await;

        assert_eq!(Track::default(), queue.next().await.unwrap());
        assert_eq!(Track::default(), queue.next().await.unwrap());
        assert_eq!(None, queue.next().await);
    }

    #[tokio::test]
    async fn next_empty_queue() {
        let queue = Queue::new();

        assert_eq!(None, queue.next().await);
    }

    #[tokio::test]
    async fn current() {
        let queue = Queue::new();
        let expected_first = 1;
        let expected_second = 2;
        queue.add(Track::default()).await;
        queue.add(Track::default()).await;

        queue.next().await;
        let first_index = queue.current();

        queue.next().await;
        let second_index = queue.current();

        assert_eq!(expected_first, first_index);
        assert_eq!(expected_second, second_index)
    }

    #[tokio::test]
    async fn current_empty_queue() {
        let queue = Queue::new();
        let expected = 0;

        let first_index = queue.current();
        queue.next().await;
        let secod_index = queue.current();

        assert_eq!(expected, first_index);
        assert_eq!(expected, secod_index);
    }

    #[tokio::test]
    async fn get_playlist() {
        let queue = Queue::new();
        let mut expected = VecDeque::new();
        queue.add(Track::default()).await;
        queue.add(Track::default()).await;
        expected.push_back(Track::default());
        expected.push_back(Track::default());

        let actual = queue.get_playlist().await;

        assert_eq!(expected, actual);
    }
}
