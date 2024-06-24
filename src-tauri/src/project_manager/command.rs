use log::error;
use tauri::State;

use super::{project_info::ProjectInfos, ProjectManagerWarpper};

#[tauri::command]
pub fn project_manager_init_watcher(state: State<ProjectManagerWarpper>) {
    state.write().rewatch().unwrap_or_else(|e| error!("{e:?}"));
}

#[tauri::command]
pub fn project_manager_project_infos(state: State<ProjectManagerWarpper>) -> ProjectInfos {
    state.read().project_infos.read().clone()
}
