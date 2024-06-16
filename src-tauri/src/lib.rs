#![deny(clippy::all, clippy::pedantic)]
#![warn(clippy::nursery)]
#![allow(clippy::missing_panics_doc)]
mod path_resolver;
mod proot;
mod res;
mod setup_process;

use tauri_plugin_log::{Target, TargetKind};

use setup_process::init_resources;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(
            tauri_plugin_log::Builder::new()
                .targets([
                    Target::new(TargetKind::Stdout),
                    Target::new(TargetKind::Stderr),
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
