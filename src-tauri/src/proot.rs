use std::fs;

use anyhow::Result;
use tar::Archive;
use xz2::read::XzDecoder;

use crate::{path_resolver::PathResolver, res::Resources};

pub fn setup_rootfs(path_resolver: &PathResolver) -> Result<()> {
    let proot_root = path_resolver.proot_root_dir();

    if proot_root
        .read_dir()
        .ok()
        .and_then(|mut iter| iter.next())
        .is_none()
    // Check whether the folder is empty or does not exist
    {
        let _ = fs::create_dir(&proot_root);

        // start unpack xz
        let rootfs_tar_xz = Resources::get("rootfs/archlinux-aarch64-rootfs.tar.xz")
            .unwrap()
            .data;
        let rootfs_tar_unxz = XzDecoder::new(rootfs_tar_xz.as_ref());

        // start unpack tar
        let mut rootfs_untar = Archive::new(rootfs_tar_unxz);
        rootfs_untar.unpack(proot_root)?;
    }

    Ok(())
}
