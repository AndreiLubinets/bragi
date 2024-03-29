use std::{
    any::{Any, TypeId},
    path::PathBuf,
};

use audiotags::Id3v2Tag;
use log::{error, warn};
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
        let track = match audiotags::Tag::new().read_from_path(&path_to_file) {
            Ok(tags) => {
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

                let duration = if tags.type_id() == TypeId::of::<Id3v2Tag>() {
                    mp3_duration::from_path(&path_to_file)
                        .ok()
                        .map(|d| d.as_secs_f64())
                } else {
                    tags.duration()
                };

                let track = Track {
                    title,
                    artist: tags.artist().map(|artist| artist.to_string()),
                    album: tags.album().map(|album| album.title.to_string()),
                    length: duration,
                    path: path_to_file,
                };

                Ok(track)
            }
            Err(err) => match err {
                audiotags::Error::Mp4TagError(_)
                | audiotags::Error::FlacTagError(_)
                | audiotags::Error::Id3TagError(_) => {
                    error!(
                        "Unable to read track metadata for {}: {}",
                        &path_to_file.display(),
                        err
                    );
                    let track = Track {
                        title: path_to_file
                            .file_stem()
                            .unwrap_or_default()
                            .to_str()
                            .unwrap_or_default()
                            .to_string(),
                        path: path_to_file,
                        ..Default::default()
                    };

                    Ok(track)
                }
                _ => Err(err),
            },
        }?;

        if track.length.is_none() {
            warn!("Unable to read track length for {}", &track.title);
        }

        Ok(track)
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }
}

#[cfg(test)]
mod tests {
    use std::{fs::File, path::PathBuf, str::FromStr};

    use audiotags::{Album, AudioTagEdit, AudioTagWrite};
    use temp_dir::TempDir;

    use super::Track;

    #[test]
    fn try_new_track_mp3() {
        let dir = TempDir::new().unwrap();
        let file_path = dir.path().join("track.mp3");
        let mut file = File::create(&file_path).unwrap();

        let expected = Track {
            title: "title".to_owned(),
            artist: Some("artist".to_owned()),
            album: Some("album".to_owned()),
            path: file_path.clone(),
            //Duration is ignored
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
    fn try_new_track_flac() {
        let dir = TempDir::new().unwrap();
        let file_path = dir.path().join("track.flac");
        let mut file = File::create(&file_path).unwrap();

        let expected = Track {
            title: "title".to_owned(),
            artist: Some("artist".to_owned()),
            album: Some("album".to_owned()),
            path: file_path.clone(),
            //Duration is ignored
            length: None,
        };

        let mut tags = audiotags::FlacTag::new();
        tags.set_title(&expected.title);
        tags.set_artist(&expected.artist.clone().unwrap());
        tags.set_album(Album::with_title(&expected.album.clone().unwrap()));
        tags.write_to(&mut file).unwrap();

        let actual = Track::try_new(&file_path).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn try_new_track_empty_tags_mp3() {
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
    fn try_new_track_empty_tags_flac() {
        let dir = TempDir::new().unwrap();
        let file_path = dir.path().join("track.flac");
        let mut file = File::create(&file_path).unwrap();

        let expected = Track {
            title: "track".to_owned(),
            artist: None,
            album: None,
            path: file_path.clone(),
            length: None,
        };

        let mut tags = audiotags::FlacTag::new();
        tags.write_to(&mut file).unwrap();

        let actual = Track::try_new(&file_path).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn new_track_no_tag() {
        let dir = TempDir::new().unwrap();
        let file_path = dir.path().join("track.mp3");
        let _ = File::create(&file_path).unwrap();

        let expected = Track {
            title: "track".to_owned(),
            artist: None,
            album: None,
            path: file_path.clone(),
            length: None,
        };

        let actual = Track::try_new(&file_path).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    #[should_panic]
    fn try_new_track_file_not_found() {
        Track::try_new(PathBuf::from_str("track").unwrap()).unwrap();
    }
}
