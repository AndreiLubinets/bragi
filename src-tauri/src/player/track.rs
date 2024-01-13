use std::{path::PathBuf, time::Duration};

pub struct Track {
    path: PathBuf,
    name: String,
    length: Duration,
}

impl Track {
    pub fn new(path: impl Into<PathBuf>, length: impl Into<Duration>) -> Self {
        let converted_path: PathBuf = path.into();
        let name = converted_path
            .file_stem()
            .unwrap_or_default()
            .to_str()
            .unwrap_or_default()
            .to_string();
        Track {
            path: converted_path,
            name,
            length: length.into(),
        }
    }
}
