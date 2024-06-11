use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::Result;
use libc::getpid;

pub fn private_android_data() -> PathBuf {
    let pid = unsafe { getpid() };
    let package = get_package_name(pid).unwrap();

    Path::new("/data/data").join(package)
}

fn get_package_name(pid: i32) -> Result<String> {
    let cmdline = Path::new("/proc").join(pid.to_string()).join("cmdline");
    let cmdline = fs::read_to_string(cmdline)?;
    let cmdline = cmdline.split(':').next().unwrap_or_default();
    Ok(cmdline.trim_matches(['\0']).trim().to_string())
}
