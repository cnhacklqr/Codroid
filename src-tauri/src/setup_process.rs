use std::collections::HashMap;

use log::error;
use serde::Serialize;
use specta::Type;
use tauri::AppHandle;
use tauri_specta::Event;

type StepFn = dyn FnOnce();
pub struct Builder {
    max_step: i32,
    steps: HashMap<i32, (String, Box<StepFn>)>,
}

impl Builder {
    pub fn new() -> Self {
        Self {
            max_step: 0,
            steps: HashMap::new(),
        }
    }

    pub fn next_step<F>(mut self, msg: String, step: F) -> Self
    where
        F: FnOnce() + 'static,
    {
        self.max_step += 1;
        self.steps.insert(self.max_step, (msg, Box::new(step)));
        self
    }

    pub fn run(self, app: &AppHandle) {
        SetupProcess {
            current_step: 0,
            max_step: self.max_step,
            message: "Setup Process".into(),
        }
        .emit(app)
        .unwrap_or_else(|e| error!("{e:?}"));

        for (step, (msg, task)) in self.steps {
            SetupProcess {
                current_step: step,
                max_step: self.max_step,
                message: msg,
            }
            .emit(app)
            .unwrap_or_else(|e| error!("{e:?}"));
            task();
        }
    }
}

#[derive(Clone, Serialize, Type, Event)]
#[serde(rename_all = "camelCase")]
pub struct SetupProcess {
    current_step: i32,
    max_step: i32,
    message: String,
}
