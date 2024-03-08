use log::info;
use std::{collections::VecDeque, io::BufReader, path::Path, path::PathBuf, time::Duration};

use rodio::{OutputStream, Sink};
use tauri::async_runtime::RwLock;

use self::{playtime::Playtime, track::Track};

mod playtime;
pub mod track;

pub struct Player {
    _stream: StreamWrapper,
    sink: Sink,
    playlist: RwLock<VecDeque<Track>>,
    playtime: RwLock<Playtime>,
}

impl Player {
    pub fn new() -> anyhow::Result<Self> {
        let (stream, handle) = OutputStream::try_default()?;

        let sink = Sink::try_new(&handle)?;
        let player = Self {
            _stream: StreamWrapper(stream),
            sink,
            playlist: RwLock::new(VecDeque::new()),
            playtime: RwLock::new(Playtime::default()),
        };

        Ok(player)
    }

    pub async fn open(&'_ self, path: impl Into<PathBuf>) -> anyhow::Result<()> {
        self.sink.stop();

        let path_to_file: PathBuf = path.into();
        let file = std::fs::File::open(&path_to_file)?;
        self.add_to_playlist(&path_to_file).await?;

        self.sink.append(rodio::Decoder::new(BufReader::new(file))?);
        self.play().await;

        info!("Playing {}", path_to_file.to_string_lossy());

        self.sink.sleep_until_end();
        self.stop().await;

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

    pub fn is_playing(&self) -> bool {
        !(self.sink.is_paused() || self.sink.empty())
    }

    pub fn get_playlist(&self) -> VecDeque<Track> {
        self.playlist.blocking_read().clone()
    }

    async fn add_to_playlist(&self, path: impl AsRef<Path>) -> anyhow::Result<()> {
        self.playlist
            .write()
            .await
            .push_front(Track::try_new(path.as_ref())?);
        Ok(())
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
