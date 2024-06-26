use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Clone, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ProjectInfos {
    pub infos: HashMap<String, ProjectInfo>,
}

#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectInfo {
    pub name: String,
    pub template: Template,
}

#[derive(Serialize_repr, Deserialize_repr, Clone, Copy)]
#[serde(rename_all = "camelCase")]
#[repr(u16)]
pub enum Template {
    RustBinary,
    RustLibrary,
    Empty,
}
