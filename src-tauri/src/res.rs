use std::{
    fs::{self, File},
    io::{Read, Write},
    path::{Path, PathBuf},
};

use anyhow::{anyhow, Result};
use rust_embed::{Embed, EmbeddedFile};

#[derive(Embed)]
#[folder = "res"]
#[include = "**"]
#[exclude = "README.md"]
#[exclude = "sources/*"]
#[exclude = ".gitattributes"]
pub struct Resources;

impl Resources {
    // 验证内嵌资源是否和本地资源相同
    pub fn verify<P: AsRef<Path>>(emed: &EmbeddedFile, absolute_path: P) -> Result<bool> {
        let mut verification_file = File::open(absolute_path.as_ref())?;
        Ok(
            Self::read_verification_file(&mut verification_file).map_or(false, |verification| {
                emed.metadata.sha256_hash() == verification
            }),
        )
    }

    // 更新本地资源和验证文件
    fn update_file<P: AsRef<Path>>(absolute_path: P, embed_file: &EmbeddedFile) -> Result<()> {
        let absolute_path = absolute_path.as_ref();
        let verification_path = Self::verification_path(absolute_path);

        // update file
        let _ = fs::remove_file(absolute_path);
        let mut resource_file = Self::create_file_all(absolute_path)?;
        resource_file.write_all(embed_file.data.as_ref())?;

        let _ = fs::remove_file(&verification_path);
        let mut verification_file = File::create(&verification_path)?;
        verification_file.write_all(&embed_file.metadata.sha256_hash())?;

        Ok(())
    }

    fn verification_path<P: AsRef<Path>>(absolute_path: P) -> PathBuf {
        Path::new(&format!("{:?}_sha256_hash", absolute_path.as_ref())).to_path_buf()
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
            let _ = fs::create_dir_all(dir);
        }

        let file = File::create(path)?;
        Ok(file)
    }
}
