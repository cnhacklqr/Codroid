use log::error;
use serde::Serialize;
use tauri::{AppHandle, Manager};

#[derive(Clone, Serialize)]
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

        result.emit(app);
        result
    }

    pub fn next_step(&mut self, message: String, app: &AppHandle) {
        self.current_step += 1;
        self.message = message;
        self.emit(app);
    }

    fn emit(&self, app: &AppHandle) {
        app.emit("setup-process", self.clone())
            .unwrap_or_else(|e| error!("{e:?}"));
    }
}
