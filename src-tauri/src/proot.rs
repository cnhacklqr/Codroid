use std::fs::{self, File};

use anyhow::Result;
use tar::Archive;
use xz2::read::XzDecoder;

use crate::{android::private_android_data, res::Resources};

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

        let rootfs_path = Resources::resources_dir()
            .join("rootfs")
            .join("archlinux-aarch64-rootfs.tar.xz");

        // start unpack xz
        let rootfs_tar_xz = File::open(rootfs_path)?;
        let rootfs_tar_unxz = XzDecoder::new(rootfs_tar_xz);

        // start unpack tar
        let mut rootfs_untar = Archive::new(rootfs_tar_unxz);
        rootfs_untar.unpack(linux_root)?;
    }

    Ok(())
}
