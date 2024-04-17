use std::path::{Path, PathBuf};

use log::{debug, error};
use tauri::{
    api::dialog::{FileDialogBuilder, MessageDialogBuilder},
    CustomMenuItem, Manager, Menu, Submenu, WindowMenuEvent,
};

use crate::command;

const EXTENSIONS: [&str; 2] = ["mp3", "flac"];

pub fn menu() -> Menu {
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let open = CustomMenuItem::new("open".to_string(), "Open Files");
    let open_folder = CustomMenuItem::new("open_folder".to_string(), "Open Folder");
    let submenu = Submenu::new(
        "File",
        Menu::new()
            .add_item(open)
            .add_item(open_folder)
            .add_item(quit),
    );

    Menu::new().add_submenu(submenu)
}

pub fn event_handler() -> impl Fn(WindowMenuEvent) {
    |event| {
        let app = event.window().app_handle();
        match event.menu_item_id() {
            "quit" => {
                std::process::exit(0);
            }
            "open" => FileDialogBuilder::default()
                .add_filter("Audio", &EXTENSIONS)
                .pick_files(move |path_bufs| {
                    match path_bufs {
                        Some(paths) => {
                            tauri::async_runtime::spawn(async move {
                                if let Err(err) = command::play_queue(&app, paths).await {
                                    error!("{}", err);
                                };
                            });
                        }
                        None => debug!("Nothing selected"),
                    };
                }),
            "open_folder" => {
                FileDialogBuilder::default().pick_folder(move |path_buf| match path_buf {
                    Some(path) => {
                        tauri::async_runtime::spawn(async move {
                            let paths: Vec<PathBuf> = open_folder(path).unwrap();

                            if paths.is_empty() {
                                MessageDialogBuilder::new("Open Folder", "No audio files found")
                                    .show(|_| ());
                                return;
                            }

                            if let Err(err) = command::play_queue(&app, paths).await {
                                error!("{}", err);
                            };
                        });
                    }
                    None => debug!("Nothing selected"),
                })
            }
            _ => error!("Unknown event"),
        }
    }
}

/// Opens a folder and returns the list of file paths that match the valid extensions
///
/// # Arguments
///
/// * 'path' - The path to the folder
///
/// # Returns
///
/// * Vec<PathBuf> - The list of file paths
///
/// # Examples
///
/// ```
/// use bragi::menu::open_folder;
/// use std::path::Path;
/// let path = Path::new("/path/to/folder");
/// let files = open_folder(path).unwrap();
fn open_folder(path: impl AsRef<Path>) -> anyhow::Result<Vec<PathBuf>> {
    let paths = path
        .as_ref()
        .read_dir()?
        .filter_map(|file| file.ok().map(|file| file.path()))
        .filter(|path| path.extension().is_some())
        .filter(|path| {
            EXTENSIONS.contains(
                &path
                    .extension()
                    .unwrap_or_default()
                    .to_str()
                    .unwrap_or_default(),
            )
        })
        .collect();

    Ok(paths)
}

#[cfg(test)]
mod tests {
    use std::fs::{create_dir, File};

    use temp_dir::TempDir;

    use super::*;

    macro_rules! assert_vec_eq {
        ($left:expr, $right:expr) => {
            let left_set: std::collections::HashSet<_> = $left.into_iter().collect();
            let right_set: std::collections::HashSet<_> = $right.into_iter().collect();
            assert_eq!(left_set, right_set);
        };
    }

    #[test]
    fn test_open_folder() {
        let dir = TempDir::new().unwrap();
        let file_path = dir.path().join("track.mp3");
        let file_path_2 = dir.path().join("track.flac");

        File::create(&file_path).unwrap();
        File::create(&file_path_2).unwrap();
        let expected = vec![file_path, file_path_2];

        let actual = open_folder(dir.path()).unwrap();

        assert_vec_eq!(expected, actual);
    }

    #[test]
    fn test_open_folder_empty() {
        let dir = TempDir::new().unwrap();

        let actual = open_folder(dir.path()).unwrap();

        assert!(actual.is_empty());
    }

    #[test]
    fn test_open_folder_invalid() {
        let actual = open_folder("invalid");

        assert!(actual.is_err());
    }

    #[test]
    fn test_open_folder_without_extensions() {
        let dir = TempDir::new().unwrap();
        let file_path = dir.path().join("track.mp3");
        let file_path_2 = dir.path().join("track");
        let dir_2 = dir.path().join("dir");

        File::create(&file_path).unwrap();
        File::create(file_path_2).unwrap();
        create_dir(dir_2).unwrap();

        let expected = vec![file_path];

        let actual = open_folder(dir.path()).unwrap();

        assert_vec_eq!(expected, actual);
    }

    #[test]
    fn test_open_folder_invalid_extensions() {
        let dir = TempDir::new().unwrap();
        let file_path = dir.path().join("track.mp3");
        let file_path_2 = dir.path().join("track.flac");
        let file_path_3 = dir.path().join("track.exe");

        File::create(&file_path).unwrap();
        File::create(&file_path_2).unwrap();
        File::create(file_path_3).unwrap();
        let expected = vec![file_path, file_path_2];

        let actual = open_folder(dir.path()).unwrap();

        assert_vec_eq!(expected, actual);
    }
}
