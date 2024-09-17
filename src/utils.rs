use anyhow::{Context, Result};
use dirs::home_dir;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

pub fn bap_root() -> PathBuf {
    home_dir().unwrap_or_default().join(".bap")
}

pub fn versions_dir() -> PathBuf {
    bap_root().join("versions")
}

pub fn bin_dir() -> PathBuf {
    bap_root().join("bin")
}

pub fn versions_file() -> PathBuf {
    versions_dir().join("versions")
}

pub fn ensure_bap_directories() -> Result<()> {
    create_dir_if_not_exists(&bap_root())?;
    create_dir_if_not_exists(&versions_dir())?;
    create_dir_if_not_exists(&bin_dir())?;
    ensure_versions_file()?;
    Ok(())
}

fn create_dir_if_not_exists(path: &Path) -> Result<()> {
    if !path.exists() {
        fs::create_dir_all(path)
            .with_context(|| format!("Failed to create directory: {}", path.display()))?;
        println!("Created directory: {}", path.display());
    }
    Ok(())
}

fn ensure_versions_file() -> Result<()> {
    let versions_file_path = versions_file();
    if !versions_file_path.exists() {
        let mut file = fs::File::create(&versions_file_path).with_context(|| {
            format!(
                "Failed to create versions file: {}",
                versions_file_path.display()
            )
        })?;
        file.write_all(b"").with_context(|| {
            format!(
                "Failed to initialize versions file: {}",
                versions_file_path.display()
            )
        })?;
        println!("Created versions file: {}", versions_file_path.display());
    }
    Ok(())
}
