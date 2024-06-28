use log::error;
use serde::Serialize;
use specta::Type;
use tauri::AppHandle;
use tauri_specta::Event;

#[derive(Clone, Serialize, Type, Event)]
#[serde(rename_all = "camelCase")]
pub struct SetupProcess {
    current_step: i32,
    max_step: i32,
    message: String,
}

impl SetupProcess {
    pub fn new(inital_message: String, max_step: i32, app: &AppHandle) -> Self {
        let result = Self {
            current_step: 0,
            max_step,
            message: inital_message,
        };

        result.emit(app).unwrap_or_else(|e| error!("{e:?}"));
        result
    }

    pub fn next_step(&mut self, message: String, app: &AppHandle) {
        self.current_step += 1;
        self.message = message;
        self.emit(app).unwrap_or_else(|e| error!("{e:?}"));
    }
}
