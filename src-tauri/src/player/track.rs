use std::path::{Path, PathBuf};

use anyhow::bail;
use audiotags::Id3v2Tag;
use log::{debug, error, warn};
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

        if !Path::exists(&path_to_file) {
            bail!("File does not exists: {}", path_to_file.to_string_lossy())
        }

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

                let duration = if tags.to_any().downcast_ref::<Id3v2Tag>().is_some() {
                    debug!("Id3v2Tag found, using mp3_duration for duration");
                    mp3_duration::from_path(&path_to_file)
                        .inspect_err(|err| error!("{}", err))
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

                Ok::<Track, anyhow::Error>(track)
            }
            Err(err) => {
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
        }?;

        if track.length.is_none() {
            warn!("Unable to read track length for {}", &track.title);
        }

        Ok(track)
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    #[allow(dead_code)]
    pub fn length(&self) -> Option<f64> {
        self.length
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
            length: Some(0.0),
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
            length: Some(0.0),
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
    fn try_new_track_file_not_found() {
        assert!(Track::try_new(PathBuf::from_str("track.mp3").unwrap()).is_err());
    }
}
