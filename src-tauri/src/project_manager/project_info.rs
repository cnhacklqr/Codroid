use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use specta::Type;

#[derive(Clone, Serialize, Deserialize, Default, Type)]
#[serde(rename_all = "camelCase")]
pub struct ProjectInfos {
    pub infos: HashMap<String, ProjectInfo>,
}

#[derive(Clone, Deserialize, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct ProjectInfo {
    pub name: String,
    pub template: Template,
}

#[derive(Serialize, Deserialize, Type, Clone, Copy)]
#[serde(rename_all = "camelCase")]
pub enum Template {
    RustBinary,
    RustLibrary,
    Empty,
}
