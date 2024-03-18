use log::info;
use std::{
    collections::VecDeque,
    io::BufReader,
    path::Path,
    sync::{
        atomic::AtomicBool,
        mpsc::{self, Receiver, Sender},
    },
    time::Duration,
};

use rodio::{OutputStream, Sink};
use tauri::async_runtime::RwLock;

use self::{playtime::Playtime, queue::Queue, track::Track};

mod playtime;
mod queue;
pub mod track;

pub struct Player {
    _stream: StreamWrapper,
    sink: Sink,
    queue: Queue,
    playtime: RwLock<Playtime>,
    event_handler: Sender<Event>,
    is_playing: AtomicBool,
}

impl Player {
    pub fn new() -> anyhow::Result<(Self, Receiver<Event>)> {
        let (stream, handle) = OutputStream::try_default()?;

        let sink = Sink::try_new(&handle)?;
        let event_handler = mpsc::channel();
        let player = Self {
            _stream: StreamWrapper(stream),
            sink,
            queue: Queue::new(),
            playtime: RwLock::new(Playtime::default()),
            event_handler: event_handler.0,
            is_playing: AtomicBool::new(false),
        };

        Ok((player, event_handler.1))
    }

    pub async fn open(&self, path: impl AsRef<Path>) -> anyhow::Result<()> {
        self.queue.add(Track::try_new(path.as_ref())?).await;
        Ok(())
    }

    pub async fn play_queue(&self) -> anyhow::Result<()> {
        info!("Starting a queue");
        self.is_playing
            .store(true, std::sync::atomic::Ordering::Relaxed);
        while let Some(track) = self.queue.next().await {
            if !self.is_playing() {
                break;
            }

            let file = std::fs::File::open(track.path())?;
            self.sink.append(rodio::Decoder::new(BufReader::new(file))?);
            self.event_handler
                .send(Event::TrackChanged(self.queue.current()))?;
            self.play().await;

            info!("Playing {}", &track.path().to_string_lossy());

            self.sink.sleep_until_end();
            self.next().await;
        }

        self.stop().await;
        info!("Queue stopped");

        Ok(())
    }

    pub async fn play(&self) {
        self.playtime.write().await.play();
        self.sink.play();
        info!("Sink resumed");
    }

    pub async fn next(&self) {
        *self.playtime.write().await = Playtime::default();
        self.sink.stop();
        info!("Switching to next track");
    }

    pub async fn stop(&self) {
        *self.playtime.write().await = Playtime::default();
        self.sink.stop();
        self.is_playing
            .store(false, std::sync::atomic::Ordering::Relaxed);
        self.queue.reset();
        self.event_handler.send(Event::PlaybackStopped).unwrap();
        info!("Sink stopped");
    }

    pub fn pause(&self) {
        self.playtime.blocking_write().pause();
        self.sink.pause();
        info!("Sink paused");
    }

    pub fn playtime(&self) -> Duration {
        self.playtime.blocking_read().time()
    }

    pub fn is_playing(&self) -> bool {
        self.is_playing.load(std::sync::atomic::Ordering::Relaxed)
    }

    pub async fn get_playlist(&self) -> VecDeque<Track> {
        self.queue.get_playlist().await
    }

    pub fn set_volume(&self, volume: impl Into<f32>) {
        let volume_f32: f32 = volume.into();
        self.sink.set_volume(volume_f32);
        info!("Volume changed to: {}", volume_f32)
    }
}

struct StreamWrapper(OutputStream);

unsafe impl Send for StreamWrapper {}
unsafe impl Sync for StreamWrapper {}

#[derive(Clone)]
pub enum Event {
    TrackChanged(usize),
    PlaybackStopped,
}
