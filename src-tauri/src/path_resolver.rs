use std::{
    env, fs,
    path::{Path, PathBuf},
};

use crate::android::private_android_data;

pub fn init() {
    let data_dir = data_dir();
    let _ = fs::create_dir(&data_dir);
    let _ = fs::create_dir(data_dir.join("resources"));
    let _ = fs::create_dir(data_dir.join("cache"));
}

pub fn data_dir() -> PathBuf {
    if cfg!(target_os = "android") {
        private_android_data()
    } else if cfg!(target_os = "linux") {
        let home = env::var("HOME").unwrap();
        Path::new(&home).join(".codroid")
    } else {
        panic!("Unsupported platform!");
    }
}

pub fn cache_dir() -> PathBuf {
    data_dir().join("cache")
}

pub fn resources_dir() -> PathBuf {
    data_dir().join("resources")
}
