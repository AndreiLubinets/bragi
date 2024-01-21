use std::{fs::Metadata, path::PathBuf, time::Duration};

use serde::Serialize;

//#[serde_as]
#[derive(Clone, Serialize, Default)]
pub struct Track {
    title: String,
    artist: String,
    //#[serde_as(as = "serde_with::DurationSeconds<i64>")]
    length: Duration,
    path: PathBuf,
}

impl Track {
    pub fn new(path: impl Into<PathBuf>, length: impl Into<Duration>) -> Self {
        Track {
            path: path.into(),
            length: length.into(),
            ..Default::default()
        }
    }
}

impl From<Metadata> for Track {
    fn from(value: Metadata) -> Self {
        todo!()
    }
}
