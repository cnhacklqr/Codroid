#![deny(clippy::all, clippy::pedantic)]
#![warn(clippy::nursery)]
#![allow(clippy::missing_panics_doc)]
mod android;
mod path_resolver;
mod payload;
mod proot;
mod res;

use log::error;
use tauri::{AppHandle, Manager};
use tauri_plugin_log::{Target, TargetKind};

use payload::Payload;
use proot::setup_rootfs;
use res::Resources;

#[tauri::command]
async fn init_resources(app: AppHandle) {
    app.emit("setup-process", Payload::new("Checking resources".into()))
        .unwrap_or_else(|e| error!("{e:?}"));
    Resources::auto_update()
        .await
        .unwrap_or_else(|e| error!("{e:?}"));
    app.emit(
        "setup-process",
        Payload::new("Checking proot rootfs".into()),
    )
    .unwrap_or_else(|e| error!("{e:?}"));
    setup_rootfs().unwrap_or_else(|e| error!("{e:?}"));
    app.emit("setup-process", Payload::new("All done.".into()))
        .unwrap_or_else(|e| error!("{e:?}"));
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    path_resolver::init();
    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::new()
                .targets([
                    Target::new(TargetKind::Stdout),
                    Target::new(TargetKind::Folder {
                        path: path_resolver::cache_dir(),
                        file_name: None,
                    }),
                ])
                .build(),
        )
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_android_utils::init())
        .invoke_handler(tauri::generate_handler![init_resources])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
