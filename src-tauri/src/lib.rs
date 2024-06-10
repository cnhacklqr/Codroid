#![deny(clippy::all, clippy::pedantic)]
#![warn(clippy::nursery)]
#![allow(clippy::missing_panics_doc)]
mod android;
mod payload;
mod proot;
mod res;

use tauri::{async_runtime, AppHandle, Manager};

use payload::Payload;
use proot::setup_rootfs;
use res::Resources;

#[tauri::command]
async fn init_resources(app: AppHandle) {
    app.emit("setup-process", Payload::new("Checking resources".into()))
        .unwrap();
    Resources::auto_update().await.unwrap();
    app.emit(
        "setup-process",
        Payload::new("Checking proot rootfs".into()),
    )
    .unwrap();
    setup_rootfs().unwrap();
    app.emit("setup-process", Payload::new("All done.".into()))
        .unwrap();
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![init_resources])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
