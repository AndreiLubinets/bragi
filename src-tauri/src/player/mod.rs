use log::{error, info};
use std::{
    collections::VecDeque,
    io::BufReader,
    path::{Path, PathBuf},
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
}

impl Player {
    pub fn new() -> anyhow::Result<Self> {
        let (stream, handle) = OutputStream::try_default()?;

        let sink = Sink::try_new(&handle)?;
        let player = Self {
            _stream: StreamWrapper(stream),
            sink,
            queue: Queue::new(),
            playtime: RwLock::new(Playtime::default()),
        };

        Ok(player)
    }

    pub async fn open(&self, path: impl AsRef<Path>) -> anyhow::Result<()> {
        self.queue.add(Track::try_new(path.as_ref())?).await;
        Ok(())
    }

    pub async fn play_queue(&self) -> anyhow::Result<()> {
        while let Some(track) = self.queue.next().await {
            let file = std::fs::File::open(&track.path())?;
            self.sink.append(rodio::Decoder::new(BufReader::new(file))?);
            self.play().await;

            info!("Playing {}", &track.path().to_string_lossy());

            self.sink.sleep_until_end();
            self.stop().await;
        }

        Ok(())
    }

    pub async fn play(&self) {
        self.playtime.write().await.play();
        self.sink.play();
        info!("Sink resumed");
    }

    pub async fn stop(&self) {
        *self.playtime.write().await = Playtime::default();
        self.sink.stop();
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

    pub async fn is_playing(&self) -> bool {
        self.playtime.read().await.time() != Duration::ZERO
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
