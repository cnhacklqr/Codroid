use std::{fs, path::PathBuf, sync::Mutex};

use crate::path_resolver::PathResolver;
use tauri::{AppHandle, Manager};
pub mod command;
mod settings_item;
use settings_item::{SettingItems, Value};

pub struct SettingsManagerWrapper {
    pub settings: Mutex<SettingsManager>,
}
pub struct SettingsManager {
    pub app: AppHandle,
    pub setting_items: SettingItems,
}

impl SettingsManager {
    pub fn managed_by(app: &AppHandle) {
        app.manage(SettingsManagerWrapper {
            settings: Mutex::new(Self {
                app: app.clone(),
                setting_items: Self::get_setting_items(app.clone()),
            }),
        });
    }

    pub fn get_setting_items(app: AppHandle) -> SettingItems {
        let path_resolver = PathResolver::new(app.clone()); // init path_resolver
        if !path_resolver
            .settings_dir()
            .join("user_settings.toml")
            .exists()
        {
            let _ = fs::copy(
                path_resolver.settings_dir().join("setting_items.toml"),
                path_resolver.settings_dir().join("user_settings.toml"),
            );
        }
        SettingItems {
            items: settings_item::Table::from_toml_table(
                fs::read_to_string(path_resolver.settings_dir().join("user_settings.toml"))
                    .unwrap()
                    .parse::<toml::Table>()
                    .unwrap()
                    .clone(),
            )
            .unwrap(),
        }
    }

    pub fn save_settings(&self) {
        let _ = fs::write(
            Self::settings_data_path(&self.app).join("user_settings.toml"),
            toml::to_string(&self.setting_items.items).unwrap(),
        );
    }

    pub fn revise_setting(&mut self, key: String, choose: i64) {
        match self.setting_items.items.data.get_mut(&key) {
            Some(value) => match value {
                Value::Table(table) => {
                    let mut key_ref = table.data.get_mut("choosed").unwrap();
                    key_ref = &mut Value::Integer(choose);
                }
                _ => {}
            },
            None => {}
        }
    }

    pub fn settings_data_path(app: &AppHandle) -> PathBuf {
        let path_resolver = PathResolver::new(app.clone());
        path_resolver.project_dir()
    }
}
