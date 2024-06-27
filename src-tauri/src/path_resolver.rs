use std::{
    env, fs, io,
    path::{Path, PathBuf},
};

use log::error;
use symlink::symlink_dir;
use tauri::AppHandle;
use tauri_plugin_android_utils::AndroidUtilsExt;

pub struct PathResolver {
    app: AppHandle,
}

impl PathResolver {
    pub const fn new(app: AppHandle) -> Self {
        Self { app }
    }

    pub fn setup(&self) {
        let _ = fs::create_dir(self.codroid_home());
        let _ = fs::create_dir(self.cache_dir());

        let bin_dir = self.bin_dir();

        if cfg!(target_os = "android") {
            let native_lib_dir = self.app.android_utils().native_lib_directory().unwrap();
            android_recreate_symlink_dir(native_lib_dir, bin_dir)
                .unwrap_or_else(|e| error!("{e:?}")); // api28后只有nativelib文件夹可以有可执行权限
        } else if !bin_dir.exists() {
            fs::create_dir(bin_dir).unwrap_or_else(|e| error!("{e:?}"));
        }

        let project_dir = self.project_dir();

        #[cfg(target_os = "android")]
        {
            let proot_home = self.proot_home_dir();
            let proot_project_dir = proot_home.join("projects");

            if !proot_project_dir.exists() {
                fs::create_dir(&proot_project_dir).unwrap_or_else(|e| error!("{e:?}"));
            }

            android_recreate_symlink_dir(proot_project_dir, project_dir)
                .unwrap_or_else(|e| error!("{e:?}"));
        }

        #[cfg(not(target_os = "android"))]
        {
            if !project_dir.exists() {
                fs::create_dir(self.project_dir()).unwrap_or_else(|e| error!("{e:?}"));
            }
        }
    }

    pub fn codroid_home(&self) -> PathBuf {
        if cfg!(target_os = "android") {
            self.app
                .android_utils()
                .private_directory()
                .unwrap()
                .join(".codroid")
        } else if cfg!(target_os = "linux") {
            let home = env::var("HOME").unwrap();
            Path::new(&home).join(".codroid")
        } else if cfg!(target_os = "windows") {
            let home = env::var("HOMEPATH").unwrap();
            Path::new(&home).join(".codroid")
        } else {
            panic!("Unsupported platform!");
        }
    }

    pub fn cache_dir(&self) -> PathBuf {
        if cfg!(target_os = "android") {
            self.app.android_utils().cache_directory().unwrap()
        } else if cfg!(target_os = "linux") {
            let home = env::var("HOME").unwrap();
            Path::new(&home).join(".cache").join("codroid")
        } else if cfg!(target_os = "windows") {
            let home = env::var("HOMEPATH").unwrap();
            Path::new(&home).join(".cache").join("codroid")
        } else {
            panic!("Unsupported platform!");
        }
    }

    pub fn bin_dir(&self) -> PathBuf {
        self.codroid_home().join("bin")
    }

    pub fn project_dir(&self) -> PathBuf {
        self.codroid_home().join("projects")
    }

    pub fn settings_dir(&self) -> PathBuf {
        self.codroid_home().join("settings")
    }

    #[cfg(target_os = "android")]
    pub fn proot_root_dir(&self) -> PathBuf {
        self.codroid_home()
            .join("proot_root")
            .join("archlinux-aarch64")
    }

    #[cfg(target_os = "android")]
    pub fn proot_home_dir(&self) -> PathBuf {
        self.proot_root_dir().join("root")
    }
}

fn android_recreate_symlink_dir<P, Q>(src: P, dst: Q) -> io::Result<()>
where
    P: AsRef<Path>,
    Q: AsRef<Path>,
{
    let _ = fs::remove_file(dst.as_ref());
    symlink_dir(src, dst)
}
