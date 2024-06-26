use log::error;
use tauri::State;

use super::{
    project_info::{ProjectInfo, ProjectInfos},
    ProjectManagerWarpper,
};

#[tauri::command]
pub fn project_manager_init_watcher(state: State<ProjectManagerWarpper>) {
    state.write().rewatch().unwrap_or_else(|e| error!("{e:?}"));
}

#[tauri::command]
pub fn project_manager_project_infos(state: State<ProjectManagerWarpper>) -> ProjectInfos {
    state.read().project_infos.read().clone()
}

#[tauri::command]
pub fn project_manager_create_project(
    project_info: ProjectInfo,
    state: State<ProjectManagerWarpper>,
) {
    state
        .read()
        .project_infos
        .write()
        .infos
        .insert(project_info.name.clone(), project_info);
}

#[tauri::command]
pub fn project_manager_remove_project(project_name: String, state: State<ProjectManagerWarpper>) {
    state
        .read()
        .project_infos
        .write()
        .infos
        .remove(&project_name);
}
