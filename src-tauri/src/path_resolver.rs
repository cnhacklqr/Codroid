use std::{
    env, fs,
    path::{Path, PathBuf},
};

use symlink::symlink_dir;
use tauri::AppHandle;
use tauri_plugin_android_utils::AndroidUtilsExt;

pub struct PathResolver {
    app: AppHandle,
}

impl PathResolver {
    pub fn new(app: AppHandle) -> Self {
        Self { app }
    }

    pub fn setup(&self) {
        let _ = fs::create_dir(self.codroid_home());
        let _ = fs::create_dir(self.cache_dir());

        let bin_dir = self.bin_dir();
        if !bin_dir.exists() {
            if cfg!(target_os = "android") {
                let native_lib_dir = self.app.android_utils().native_lib_directory().unwrap();
                let _ = fs::remove_file(&bin_dir); // 修复: 不是第一次安装时下一行会panic(因为native_lib_dir安装时会变动?未求证)
                symlink_dir(native_lib_dir, bin_dir).unwrap(); // api28后只有nativelib文件夹可以有可执行权限
            } else {
                let _ = fs::create_dir(bin_dir);
            }
        }

        #[cfg(target_os = "android")]
        {
            let project_dir = self.project_dir();
            if !project_dir.exists() {
                let _ = fs::remove_file(&project_dir);
                let proot_home = self.proot_home_dir();
                let proot_project_dir = proot_home.join("projects");
                let _ = fs::create_dir(&proot_project_dir);
                symlink_dir(proot_project_dir, project_dir).unwrap();
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

    #[cfg(target_os = "android")]
    pub fn proot_root_dir(&self) -> PathBuf {
        self.codroid_home().join("proot_root")
    }

    #[cfg(target_os = "android")]
    pub fn proot_home_dir(&self) -> PathBuf {
        self.proot_root_dir().join("root")
    }
}
