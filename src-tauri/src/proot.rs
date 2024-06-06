use std::fs::{self, File};

use zip::ZipArchive;

use crate::{
    android::{private_android_cache, private_android_data},
    res::Resources,
};

pub fn setup_rootfs() {
    let linux_root = private_android_data().join("linux_root");

    if linux_root
        .read_dir()
        .ok()
        .and_then(|mut iter| iter.next())
        .is_some()
    // Check whether the folder is empty or does not exist
    {
        let rootfs_path = Resources::resources_dir()
            .join("rootfs")
            .join("archlinux-aarch64-rootfs.tar.xz");
        let mut rootfs = File::open(rootfs_path).unwrap();
        let mut rootfs = ZipArchive::new(rootfs).unwrap();

        let cache_dir = private_android_cache();
        rootfs.extract(&cache_dir).unwrap();
    }
}
