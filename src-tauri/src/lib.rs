#![deny(clippy::all, clippy::pedantic)]
#![warn(clippy::nursery)]
#![allow(clippy::missing_panics_doc)]
mod path_resolver;
mod project_manager;
#[cfg(target_os = "android")]
mod proot;
mod res;
mod setup_process;

use anyhow::Result;
use log::error;
use tauri::{ipc::Invoke, App, AppHandle, Manager};
use tauri_plugin_log::{Target, TargetKind};

use path_resolver::PathResolver;
use project_manager::{
    command::{
        project_manager_create_project, project_manager_init_watcher,
        project_manager_project_infos, project_manager_remove_project,
    },
    ProjectManager, ProjectManagerUpdate,
};
#[cfg(target_os = "android")]
use proot::setup_rootfs;
use setup_process::SetupProcess;

#[tauri::command]
#[specta::specta]
async fn init(app: AppHandle) {
    let path_resolver = PathResolver::new(app.clone());

    let mut stepper = SetupProcess::new("Setup Process".into(), 3, &app);

    stepper.next_step("Setting Home Directory".into(), &app); // 1
    path_resolver.setup();

    #[cfg(target_os = "android")]
    {
        stepper.next_step("Checking proot rootfs".into(), &app); // 2
        setup_rootfs(&path_resolver).unwrap_or_else(|e| error!("{e:?}"));
    }

    stepper.next_step("All Done".into(), &app); // 3

    ProjectManager::managed_by(app.app_handle()).unwrap_or_else(|e| error!("{e:?}"));
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let (invoke_handler, register_events) = build_tauri_specta(false).unwrap();

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
        .invoke_handler(invoke_handler)
        .setup(|app| {
            register_events(app);
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn build_tauri_specta(export: bool) -> Result<(impl Fn(Invoke) -> bool, impl FnOnce(&App))> {
    let mut builder = tauri_specta::ts::builder()
        .commands(tauri_specta::collect_commands![
            init,
            project_manager_project_infos,
            project_manager_init_watcher,
            project_manager_create_project,
            project_manager_remove_project,
        ])
        .events(tauri_specta::collect_events![
            SetupProcess,
            ProjectManagerUpdate
        ]);

    if export {
        builder = builder.path("../src/bindings.ts").header("// @ts-nocheck");
    }

    let result = builder.build()?;
    Ok(result)
}

#[test]
fn export_types() {
    let _ = build_tauri_specta(true).unwrap();
}
