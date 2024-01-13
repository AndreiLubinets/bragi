use std::{any::Any, error::Error, fs::File, io::BufReader, path::PathBuf, time::Duration};

use rodio::{OutputStream, Sink, Source};

use self::track::Track;

mod playtime;
mod track;

pub struct Player {
    _stream: StreamWrapper,
    sink: Sink,
    playlist: Vec<Track>,
}

impl Player {
    pub async fn play(&'_ self, path: impl Into<PathBuf>) -> Result<(), Box<dyn Error + '_>> {
        self.sink.stop();
        let file = std::fs::File::open(path.into())?;
        self.sink.append(rodio::Decoder::new(BufReader::new(file))?);
        self.sink.play();

        println!("Playing");

        self.sink.sleep_until_end();

        Ok(())
    }

    pub fn start(&self) {
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

    pub fn playtime(&self) {}

    pub fn is_playing(&self) -> bool {
        !(self.sink.is_paused() || self.sink.empty())
    }

    pub fn get_playlist(&self) -> &[Track] {
        self.playlist.as_slice()
    }

    /*fn add_to_playlist(&self, path: PathBuf) {
        self.playlist
            .push(Track::new(path, Duration::from_secs(15)))
    }*/
}

impl Default for Player {
    fn default() -> Self {
        let (stream, handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&handle).unwrap();
        Self {
            _stream: StreamWrapper(stream),
            sink,
            playlist: vec![],
        }
    }
}

struct StreamWrapper(OutputStream);

unsafe impl Send for StreamWrapper {}
unsafe impl Sync for StreamWrapper {}
