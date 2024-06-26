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
        .update_project_infos(|project_infos| {
            project_infos
                .infos
                .insert(project_info.name.clone(), project_info);
        })
        .unwrap_or_else(|e| error!("{e:?}"));
}

#[tauri::command]
pub fn project_manager_remove_project(project_name: String, state: State<ProjectManagerWarpper>) {
    state
        .read()
        .update_project_infos(|project_infos| {
            project_infos.infos.remove(&project_name);
        })
        .unwrap_or_else(|e| error!("{e:?}"));
}
