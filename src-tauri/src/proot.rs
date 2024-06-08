use std::{
    fs::{self, File},
    io,
};

use anyhow::Result;
use tar::Archive;
use xz2::read::XzDecoder;

use crate::{
    android::{private_android_cache, private_android_data},
    res::Resources,
};

pub fn setup_rootfs() -> Result<()> {
    let linux_root = private_android_data().join("linux_root");

    if linux_root
        .read_dir()
        .ok()
        .and_then(|mut iter| iter.next())
        .is_none()
    // Check whether the folder is empty or does not exist
    {
        let _ = fs::create_dir(&linux_root);

        let cache_dir = private_android_cache();
        let rootfs_path = Resources::resources_dir()
            .join("rootfs")
            .join("archlinux-aarch64-rootfs.tar.xz");

        // start unpack xz
        let rootfs_tar_xz = File::open(rootfs_path)?;
        let mut rootfs_tar_unxz = XzDecoder::new(rootfs_tar_xz);

        let rootfs_tar = cache_dir.join("archlinux-aarch64-rootfs.tar");
        let _ = fs::remove_file(&rootfs_tar);
        let mut rootfs_tar = File::create(rootfs_tar)?;
        io::copy(&mut rootfs_tar_unxz, &mut rootfs_tar)?;

        // start unpack tar
        let mut rootfs_untar = Archive::new(rootfs_tar);
        rootfs_untar.unpack(linux_root)?;
    }

    Ok(())
}
