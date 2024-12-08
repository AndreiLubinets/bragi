use std::{
    collections::VecDeque,
    sync::atomic::{AtomicUsize, Ordering},
};

use log::info;
use tauri::async_runtime::Mutex;

use crate::util::AtomicSub;

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
        self.tracks
            .lock()
            .await
            .get(self.current.fetch_add(1, Ordering::Relaxed))
            .cloned()
    }

    pub fn current(&self) -> usize {
        let index = self.current.load(Ordering::Relaxed);
        if index == 0 {
            return index;
        }
        index - 1
    }

    pub async fn change_current(&self, index: usize) -> anyhow::Result<()> {
        if index >= self.tracks.lock().await.len() {
            anyhow::bail!("Invalid index: {}", index);
        }

        self.current.store(index, Ordering::Relaxed);

        Ok(())
    }

    pub async fn change_to_previous(&self) {
        //requies to substract 2 so the call of next() method
        //during iteration will return previous track
        self.current.saturating_sub(2);
    }

    pub async fn current_track(&self) -> Option<Track> {
        self.tracks.lock().await.get(self.current()).cloned()
    }

    //TODO: Remove clone
    pub async fn get_playlist(&self) -> VecDeque<Track> {
        self.tracks.lock().await.clone()
    }

    pub fn reset(&self) {
        self.current.store(0, Ordering::Relaxed);
    }
}

#[cfg(test)]
mod tests {
    use tokio::test;

    use crate::player::track::Track;
    use std::collections::VecDeque;

    use super::Queue;

    #[test]
    async fn add() {
        let queue = Queue::new();
        queue.add(Track::default()).await;

        assert_eq!(Track::default(), queue.next().await.unwrap());
    }

    #[test]
    async fn next() {
        let queue = Queue::new();
        queue.add(Track::default()).await;
        queue.add(Track::default()).await;

        assert_eq!(Track::default(), queue.next().await.unwrap());
        assert_eq!(Track::default(), queue.next().await.unwrap());
        assert_eq!(None, queue.next().await);
    }

    #[test]
    async fn next_empty_queue() {
        let queue = Queue::new();

        assert_eq!(None, queue.next().await);
    }

    #[test]
    async fn current() {
        let queue = Queue::new();
        let expected_first = 0;
        let expected_second = 1;
        queue.add(Track::default()).await;
        queue.add(Track::default()).await;

        queue.next().await;
        let first_index = queue.current();

        queue.next().await;
        let second_index = queue.current();

        assert_eq!(expected_first, first_index);
        assert_eq!(expected_second, second_index)
    }

    #[test]
    async fn current_empty_queue() {
        let queue = Queue::new();
        let expected = 0;

        let first_index = queue.current();
        queue.next().await;
        let secod_index = queue.current();

        assert_eq!(expected, first_index);
        assert_eq!(expected, secod_index);
    }

    #[test]
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

    #[test]
    async fn reset() {
        let queue = Queue::new();
        queue.add(Track::default()).await;
        queue.add(Track::default()).await;

        queue.next().await;
        queue.next().await;

        queue.reset();

        assert_eq!(0, queue.current());
    }

    #[test]
    async fn change_current() {
        let queue = Queue::new();
        queue.add(Track::default()).await;
        queue.add(Track::default()).await;

        queue.change_current(1).await.unwrap();
        queue.next().await;

        assert_eq!(1, queue.current());
    }

    #[test]
    async fn change_current_invalid_index() {
        let queue = Queue::new();
        queue.add(Track::default()).await;

        assert!(queue.change_current(1).await.is_err());
    }

    #[test]
    async fn current_track() {
        let queue = Queue::new();
        queue.add(Track::default()).await;

        assert_eq!(Track::default(), queue.current_track().await.unwrap());
    }

    #[test]
    async fn current_track_empty_queue() {
        let queue = Queue::new();

        assert_eq!(None, queue.current_track().await);
    }

    #[test]
    async fn change_to_previous_test() {
        let queue = Queue::new();
        queue.add(Track::default()).await;
        queue.add(Track::default()).await;

        queue.next().await;
        queue.next().await;
        let first_index = queue.current();

        queue.change_to_previous().await;

        let second_index = queue.current();

        assert_eq!(1, first_index);
        assert_eq!(0, second_index);
    }

    #[test]
    async fn change_to_previous_invalid_index() {
        let queue = Queue::new();
        queue.add(Track::default()).await;

        queue.next().await;
        queue.change_to_previous().await;

        let index = queue.current();

        assert_eq!(0, index);
    }

    #[test]
    async fn change_to_previous_empty_queue() {
        let queue = Queue::new();

        queue.change_to_previous().await;
        let index = queue.current();

        assert_eq!(0, index);
    }
}
