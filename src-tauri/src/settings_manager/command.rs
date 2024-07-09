use tauri::{AppHandle, State};

use super::settings_item::SettingItems;
use super::{SettingsManager, SettingsManagerWrapper};

#[tauri::command]
#[specta::specta]
pub fn settings_manager_get_setting_items(app: AppHandle) -> SettingItems {
    SettingsManager::get_setting_items(app)
}

#[tauri::command]
#[specta::specta]
pub fn settings_manager_save_settings(state: State<SettingsManagerWrapper>) {
    state.settings.lock().unwrap().save_settings();
}

#[tauri::command]
#[specta::specta]
pub fn settings_manager_revise_setting(
    state: State<SettingsManagerWrapper>,
    key: String,
    choose: i64,
) {
    state.settings.lock().unwrap().revise_setting(key, choose);
}
