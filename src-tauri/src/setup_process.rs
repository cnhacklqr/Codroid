use log::error;
use serde::Serialize;
use tauri::{AppHandle, Manager};

use crate::{path_resolver::PathResolver, proot::setup_rootfs};

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

#[tauri::command]
pub async fn init_resources(app: AppHandle) {
    let path_resolver = PathResolver::new(app.clone());

    let mut stepper = SetupProcess::new("Setup Process".into(), 3, &app);

    stepper.next_step("Setting Home Directory".into(), &app); // 1
    path_resolver.setup();

    stepper.next_step("Checking proot rootfs".into(), &app); // 2
    setup_rootfs(&path_resolver).unwrap_or_else(|e| error!("{e:?}"));

    stepper.next_step("All Done".into(), &app); // 3
}
