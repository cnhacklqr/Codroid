use std::fs::{self, File};

use anyhow::Result;
use zip::ZipArchive;

use crate::{
    android::{private_android_cache, private_android_data},
    res::Resources,
};

pub async fn setup_rootfs() -> Result<()> {
    let linux_root = private_android_data().join("linux_root");

    if linux_root
        .read_dir()
        .ok()
        .and_then(|mut iter| iter.next())
        .is_none()
    // Check whether the folder is empty or does not exist
    {
        fs::create_dir(&linux_root)?;
        let rootfs_path = Resources::resources_dir()
            .join("rootfs")
            .join("archlinux-aarch64-rootfs.tar.xz");
        let rootfs = File::open(rootfs_path)?;
        let mut rootfs = ZipArchive::new(rootfs)?;

        let cache_dir = private_android_cache();
        rootfs.extract(&cache_dir)?;
    }

    Ok(())
}
