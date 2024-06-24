#![deny(clippy::all, clippy::pedantic)]
#![warn(clippy::nursery)]
#![allow(clippy::missing_panics_doc)]
mod path_resolver;
mod project_manager;
#[cfg(target_os = "android")]
mod proot;
mod res;
mod setup_process;

use log::error;
use project_manager::ProjectManager;
use tauri::Manager;
use tauri_plugin_log::{Target, TargetKind};

use project_manager::command::{project_manager_init_watcher, project_manager_project_infos};
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
        .invoke_handler(tauri::generate_handler![
            init_resources,
            project_manager_project_infos,
            project_manager_init_watcher
        ])
        .setup(|app| {
            ProjectManager::managed_by(app.app_handle()).unwrap_or_else(|e| error!("{e:?}"));
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
