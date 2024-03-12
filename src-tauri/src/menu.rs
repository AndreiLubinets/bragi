use log::{debug, error};
use tauri::{api::dialog, CustomMenuItem, Manager, Menu, Submenu, WindowMenuEvent};

use crate::command;

const EXTENSIONS: [&str; 2] = ["mp3", "flac"];

pub fn menu() -> Menu {
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let open = CustomMenuItem::new("open".to_string(), "Open");
    let submenu = Submenu::new("File", Menu::new().add_item(open).add_item(quit));

    Menu::new().add_submenu(submenu)
}

pub fn event_handler() -> impl Fn(WindowMenuEvent) {
    |event| {
        let app = event.window().app_handle();
        match event.menu_item_id() {
            "quit" => {
                std::process::exit(0);
            }
            "open" => dialog::FileDialogBuilder::default()
                .add_filter("Audio", &EXTENSIONS)
                .pick_file(move |path_buf| {
                    match path_buf {
                        Some(path) => {
                            tauri::async_runtime::spawn(async move {
                                if let Err(err) = command::play_queue(app, path).await {
                                    error!("{}", err);
                                };
                            });
                        }
                        None => debug!("Nothing selected"),
                    };
                }),
            _ => error!("Unknown event"),
        }
    }
}
