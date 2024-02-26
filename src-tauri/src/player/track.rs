use std::path::PathBuf;

use anyhow::Ok;
use serde::Serialize;

#[derive(Clone, Serialize, Default, Debug, PartialEq)]
pub struct Track {
    title: String,
    artist: Option<String>,
    album: Option<String>,
    path: PathBuf,
    length: Option<f64>,
}

impl Track {
    pub fn try_new(path: impl Into<PathBuf>) -> anyhow::Result<Self> {
        let path_to_file: PathBuf = path.into();
        let tags = audiotags::Tag::new().read_from_path(&path_to_file)?;
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

        let track = Track {
            title,
            artist: tags.artist().map(|artist| artist.to_string()),
            album: tags.album().map(|album| album.title.to_string()),
            path: path_to_file,
            length: tags.duration(),
        };

        Ok(track)
    }
}

#[cfg(test)]
mod tests {
    use std::{fs::File, path::PathBuf, str::FromStr};

    use audiotags::{Album, AudioTagEdit, AudioTagWrite};
    use temp_dir::TempDir;

    use super::Track;

    #[test]
    fn try_new_track() {
        let dir = TempDir::new().unwrap();
        let file_path = dir.path().join("track.mp3");
        let mut file = File::create(&file_path).unwrap();

        let expected = Track {
            title: "title".to_owned(),
            artist: Some("artist".to_owned()),
            album: Some("album".to_owned()),
            path: file_path.clone(),
            //TODO - Redo test to account for length
            length: None,
        };

        let mut tags = audiotags::Id3v2Tag::new();
        tags.set_title(&expected.title);
        tags.set_artist(&expected.artist.clone().unwrap());
        tags.set_album(Album::with_title(&expected.album.clone().unwrap()));
        tags.write_to(&mut file).unwrap();

        let actual = Track::try_new(&file_path).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn try_new_track_empty_tags() {
        let dir = TempDir::new().unwrap();
        let file_path = dir.path().join("track.mp3");
        let mut file = File::create(&file_path).unwrap();

        let expected = Track {
            title: "track".to_owned(),
            artist: None,
            album: None,
            path: file_path.clone(),
            length: None,
        };

        let mut tags = audiotags::Id3v2Tag::new();
        tags.write_to(&mut file).unwrap();

        let actual = Track::try_new(&file_path).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    #[should_panic]
    fn try_new_track_file_not_found() {
        Track::try_new(PathBuf::from_str("track.mp3").unwrap()).unwrap();
    }
}
