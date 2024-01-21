use std::{error::Error, io::BufReader, path::PathBuf, time::Duration};

use audiotags::Tag;
use rodio::{OutputStream, Sink};
use tauri::async_runtime::RwLock;

use self::track::Track;

mod playtime;
pub mod track;

pub struct Player {
    _stream: StreamWrapper,
    sink: Sink,
    playlist: RwLock<Vec<Track>>,
}

impl Player {
    pub async fn open(&'_ self, path: impl Into<PathBuf>) -> Result<(), Box<dyn Error + '_>> {
        self.sink.stop();

        let path_to_file: PathBuf = path.into();
        let file = std::fs::File::open(&path_to_file)?;
        self.add_to_playlist(path_to_file).await;

        self.sink.append(rodio::Decoder::new(BufReader::new(file))?);
        self.sink.play();

        println!("Playing");

        self.sink.sleep_until_end();

        Ok(())
    }

    pub fn play(&self) {
        self.sink.play();
        println!("Sink resumed");
    }

    pub fn stop(&self) {
        self.sink.stop();
        println!("Sink stopped");
    }

    pub fn pause(&self) {
        self.sink.pause();
        println!("Sink paused");
    }

    pub fn playtime(&self) {
        todo!()
    }

    pub fn is_playing(&self) -> bool {
        !(self.sink.is_paused() || self.sink.empty())
    }

    pub fn get_playlist(&self) -> Vec<Track> {
        self.playlist.blocking_read().to_vec()
    }

    async fn add_to_playlist(&self, path: PathBuf) {
        //let metadata = Tag::new().read_from_path(&path).unwrap();
        self.playlist
            .write()
            .await
            .push(Track::new(path, Duration::from_secs(350)))
    }
}

impl Default for Player {
    fn default() -> Self {
        let (stream, handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&handle).unwrap();
        Self {
            _stream: StreamWrapper(stream),
            sink,
            playlist: RwLock::new(vec![]),
        }
    }
}

struct StreamWrapper(OutputStream);

unsafe impl Send for StreamWrapper {}
unsafe impl Sync for StreamWrapper {}
