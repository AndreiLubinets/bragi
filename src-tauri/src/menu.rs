use std::path::{Path, PathBuf};

use log::{debug, error};
use tauri::{
    menu::{Menu, MenuEvent, MenuItemBuilder, SubmenuBuilder},
    AppHandle, Manager, Runtime,
};
use tauri_plugin_dialog::{DialogExt, MessageDialogKind};

use crate::command;
use crate::player::Player;

const EXTENSIONS: [&str; 2] = ["mp3", "flac"];

pub fn menu<R>() -> impl FnOnce(&AppHandle<R>) -> tauri::Result<Menu<R>> + Send
where
    R: Runtime,
{
    |app| {
        let quit = MenuItemBuilder::new("Quit").id("quit").build(app)?;
        let open = MenuItemBuilder::new("Open Files").id("open").build(app)?;
        let open_folder = MenuItemBuilder::new("Open Folder")
            .id("open_folder")
            .build(app)?;
        let submenu_file = SubmenuBuilder::new(app, "File")
            .items(&[&open, &open_folder, &quit])
            .build()?;

        let play = MenuItemBuilder::new("Play").id("play").build(app)?;
        let pause = MenuItemBuilder::new("Pause").id("pause").build(app)?;
        let stop = MenuItemBuilder::new("Stop").id("stop").build(app)?;
        let previous = MenuItemBuilder::new("Previous").id("previous").build(app)?;
        let next = MenuItemBuilder::new("Next").id("next").build(app)?;
        let submenu_playback = SubmenuBuilder::new(app, "Playback")
            .items(&[&play, &pause, &stop, &previous, &next])
            .build()?;

        let volume_up = MenuItemBuilder::new("Volume Up")
            .id("volume_up")
            .build(app)?;
        let volume_down = MenuItemBuilder::new("Volume Down")
            .id("volume_down")
            .build(app)?;
        let mute = MenuItemBuilder::new("Mute").id("mute").build(app)?;
        let submenu_volume = SubmenuBuilder::new(app, "Volume")
            .items(&[&volume_up, &volume_down, &mute])
            .build()?;

        Menu::with_items(app, &[&submenu_file, &submenu_playback, &submenu_volume])
    }
}

pub fn event_handler<R>() -> impl Fn(&AppHandle<R>, MenuEvent) + Send + Sync + 'static
where
    R: Runtime,
{
    |app: &AppHandle<R>, event: MenuEvent| {
        let handle = app.clone();
        match event.id().0.as_ref() {
            "quit" => {
                std::process::exit(0);
            }
            "open" => app
                .dialog()
                .file()
                .add_filter("Audio", &EXTENSIONS)
                .pick_files(move |file_paths| {
                    match file_paths {
                        Some(paths) => {
                            tauri::async_runtime::spawn(async move {
                                let path_bufs = paths
                                    .iter()
                                    .map(|entry| {
                                        entry.clone().into_path().expect("Not a file path")
                                    })
                                    .collect();

                                if let Err(err) = command::play_queue(&handle, path_bufs).await {
                                    error!("{}", err);
                                };
                            });
                        }
                        None => debug!("Nothing selected"),
                    };
                }),
            "open_folder" => app
                .dialog()
                .file()
                .pick_folder(move |path_buf| match path_buf {
                    Some(path) => {
                        tauri::async_runtime::spawn(async move {
                            let paths: Vec<PathBuf> = open_folder(path.as_path().unwrap()).unwrap();

                            if paths.is_empty() {
                                handle
                                    .dialog()
                                    .message("No audio files found")
                                    .title("Open Folder")
                                    .kind(MessageDialogKind::Error)
                                    .blocking_show();
                                return;
                            }

                            if let Err(err) = command::play_queue(&handle, paths).await {
                                error!("{}", err);
                            };
                        });
                    }
                    None => debug!("Nothing selected"),
                }),
            "play" => {
                tauri::async_runtime::spawn(async move {
                    if let Err(err) = command::play(handle.state::<Player>()).await {
                        error!("{}", err);
                    }
                });
            }
            "pause" => {
                command::pause(app.state::<Player>());
            }
            "stop" => {
                tauri::async_runtime::spawn(async move {
                    if let Err(err) = command::stop(handle.state::<Player>()).await {
                        error!("{}", err);
                    }
                });
            }
            "previous" => {
                tauri::async_runtime::spawn(async move {
                    if let Err(err) = command::previous_track(handle.state::<Player>()).await {
                        error!("{}", err);
                    }
                });
            }
            "next" => {
                tauri::async_runtime::spawn(async move {
                    if let Err(err) = command::next_track(handle.state::<Player>()).await {
                        error!("{}", err);
                    }
                });
            }
            "volume_up" => {
                command::adjust_volume(app.state::<Player>(), 0.06);
            }
            "volume_down" => {
                command::adjust_volume(app.state::<Player>(), -0.06);
            }
            "mute" => {
                //TODO: Fix ui update
                command::set_volume(app.state::<Player>(), 0.0);
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

    use crate::assert_vec_eq;

    use super::*;

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
