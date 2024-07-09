#![deny(clippy::all, clippy::pedantic)]
#![warn(clippy::nursery)]
#![allow(clippy::missing_panics_doc)]
mod path_resolver;
mod project_manager;
#[cfg(target_os = "android")]
mod proot;
mod res;
mod settings_manager;
mod setup_process;

use log::error;
use tauri::AppHandle;
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

macro_rules! specta_builder {
    () => {
        tauri_specta::ts::builder()
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
            ])
    };
}

#[tauri::command]
#[specta::specta]
async fn init(app: AppHandle) {
    let path_resolver = PathResolver::new(app.clone());
    let mut stepper = setup_process::Builder::new();

    {
        let path_resolver = path_resolver;
        stepper = stepper.next_step("Setting Home Directory".into(), move || {
            path_resolver.setup();
        });
    }

    #[cfg(target_os = "android")]
    {
        let path_resolver = path_resolver.clone();
        stepper = stepper.next_step("Checking proot rootfs".into(), move || {
            setup_rootfs(&path_resolver).unwrap_or_else(|e| error!("{e:?}"));
        });
    }

    {
        let app = app.clone();
        stepper = stepper.next_step("All Done".into(), move || {
            ProjectManager::managed_by(&app).unwrap_or_else(|e| error!("{e:?}"));
        });
    }

    stepper.run(&app);
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let (invoke_handler, register_events) = specta_builder!().build().unwrap();

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

#[test]
fn export_types() {
    specta_builder!()
        .path("../src/bindings.ts")
        .header("// @ts-nocheck")
        .config(specta::ts::ExportConfig::default().formatter(specta::ts::formatter::prettier))
        .export()
        .unwrap();
}
