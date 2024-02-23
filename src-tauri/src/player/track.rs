use std::path::PathBuf;

use serde::Serialize;

#[derive(Clone, Serialize, Default)]
pub struct Track {
    title: String,
    artist: Option<String>,
    album: Option<String>,
    path: PathBuf,
    length: Option<f64>,
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
            artist: tags.artist().map(|artist| artist.to_string()),
            album: tags.album().map(|album| album.title.to_string()),
            path: path_to_file,
            length: tags.duration(),
        }
    }
}
