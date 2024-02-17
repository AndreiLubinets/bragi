use std::{
    path::PathBuf,
    time::{Duration, Instant},
};

use serde::Serialize;

#[derive(Clone, Serialize, Default)]
pub struct Track {
    title: String,
    artist: String,
    album: String,
    pub path: PathBuf,
    length: f64,
    //#[serde(skip)]
    //playtime: Playtime,
}

impl Track {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        let path_to_file: PathBuf = path.into();
        let tags = audiotags::Tag::new().read_from_path(&path_to_file).unwrap();
        let title = tags
            .title()
            .unwrap_or(
                path_to_file
                    .file_stem()
                    .unwrap_or_default()
                    .to_str()
                    .unwrap_or_default(),
            )
            .to_string();

        Track {
            title,
            artist: tags.artist().unwrap_or_default().to_string(),
            album: tags
                .album()
                .map(|album| album.title.to_string())
                .unwrap_or_default(),
            path: path_to_file,
            length: tags.duration().unwrap_or_default(),
        }
    }

    /*
    pub fn pause(&mut self) {
        self.playtime.pause();
    }

    pub fn play(&mut self) {
        self.playtime.pause();
    }

    pub fn time(&self) -> Duration {
        self.playtime.time()
    }*/
}

/*
#[derive(Clone)]
struct Playtime {
    start_time: Instant,
    pause_time: Option<Instant>,
    pause_duration: Duration,
}

impl Playtime {
    pub fn pause(&mut self) {
        self.pause_time = Some(Instant::now());
    }

    pub fn play(&mut self) {
        if let Some(t) = self.pause_time.take() {
            self.pause_duration += t.elapsed();
        }
    }

    pub fn time(&self) -> Duration {
        match self.pause_time {
            Some(t) => self.start_time.elapsed() - t.elapsed() - self.pause_duration,
            None => self.start_time.elapsed() - self.pause_duration,
        }
    }
}

impl Default for Playtime {
    fn default() -> Self {
        Self {
            start_time: Instant::now(),
            pause_time: Default::default(),
            pause_duration: Default::default(),
        }
    }
}*/
