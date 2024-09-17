use crate::utils::bap_root;
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Deserialize, Serialize, Default)]
pub struct Config {
    pub default_version: Option<String>,
}

impl Config {
    pub fn load() -> Result<Self> {
        let config_path = config_file_path();
        if config_path.exists() {
            let content = fs::read_to_string(config_path)?;
            Ok(serde_json::from_str(&content)?)
        } else {
            Ok(Config::default())
        }
    }

    pub fn save(&self) -> Result<()> {
        let config_path = config_file_path();
        let content = serde_json::to_string_pretty(self)?;
        fs::write(config_path, content)?;
        Ok(())
    }
}

pub fn set_local_version(version: &str) -> Result<()> {
    fs::write(".baprc", version)?;
    Ok(())
}

pub fn get_version() -> Result<Option<String>> {
    if let Ok(version) = fs::read_to_string(".baprc") {
        return Ok(Some(version.trim().to_string()));
    }

    let config = Config::load()?;
    Ok(config.default_version)
}

fn config_file_path() -> PathBuf {
    bap_root().join("config.json")
}
