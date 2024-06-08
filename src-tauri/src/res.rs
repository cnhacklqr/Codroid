use std::{
    fs::{self, File},
    io::{Read, Write},
    path::{Path, PathBuf},
};

use anyhow::{anyhow, Result};
use rust_embed::{Embed, EmbeddedFile};
use tokio::task::JoinSet;

use crate::android::private_android_data;

#[derive(Embed)]
#[folder = "res"]
#[include = "**"]
#[exclude = "README.md"]
#[exclude = "sources/*"]
pub struct Resources;

impl Resources {
    pub fn resources_dir() -> PathBuf {
        let data_dir = private_android_data();
        data_dir.join("resources")
    }

    pub async fn auto_update() -> Result<()> {
        let resources_dir = Self::resources_dir();

        let _ = fs::create_dir(&resources_dir);
        let mut task_set = JoinSet::new();
        for relative_path in Self::iter() {
            let relative_path = relative_path.as_ref();
            let absolute_path = resources_dir.join(relative_path);
            let verification_path = resources_dir.join(format!("{relative_path}_sha256_hash"));
            let embed_file = Self::get(relative_path).unwrap();

            if absolute_path.exists() && verification_path.exists() {
                if let Ok(mut verification_file) = File::open(&verification_path) {
                    if Self::verify(&embed_file, &mut verification_file) {
                        continue; // skip this file since it's updated
                    }
                }
            }

            {
                let relative_path = relative_path.to_string();
                task_set.spawn(async move {
                    Self::update_file(absolute_path, relative_path.into(), &embed_file)
                });
            }
        }

        while let Some(result) = task_set.join_next().await {
            result??;
        }

        Ok(())
    }

    pub fn verify(emed: &EmbeddedFile, verification_file: &mut File) -> bool {
        Self::read_verification_file(verification_file).map_or(false, |verification| {
            emed.metadata.sha256_hash() == verification
        })
    }

    fn update_file<P: AsRef<Path>>(
        absolute_path: P,
        verification_path: P,
        embed_file: &EmbeddedFile,
    ) -> Result<()> {
        let absolute_path = absolute_path.as_ref();
        let verification_path = verification_path.as_ref();

        // update file
        let _ = fs::remove_file(absolute_path);
        let mut resource_file = Self::create_file_all(absolute_path)?;
        resource_file.write_all(embed_file.data.as_ref())?;

        let _ = fs::remove_file(verification_path);
        let mut verification_file = File::create(verification_path)?;
        verification_file.write_all(&embed_file.metadata.sha256_hash())?;

        Ok(())
    }

    fn read_verification_file(verification_file: &mut File) -> Result<[u8; 32]> {
        let mut verification = [0u8; 32];
        verification_file.read_exact(&mut verification)?;
        Ok(verification)
    }

    fn create_file_all<P: AsRef<Path>>(path: P) -> Result<File> {
        let path = path.as_ref();

        if path.is_dir() {
            return Err(anyhow!("The path parameter is not a file"));
        }

        if let Some(dir) = path.parent() {
            if !dir.exists() {
                fs::create_dir_all(dir)?;
            }
        }

        let file = File::create(path)?;
        Ok(file)
    }
}
