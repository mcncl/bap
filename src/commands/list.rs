use crate::config::{get_version, Config};
use crate::utils::versions_file;
use anyhow::Result;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn run() -> Result<()> {
    let versions = list_installed_versions()?;
    let current_version = get_version()?;
    let config = Config::load()?;

    if versions.is_empty() {
        println!("No Buildkite agent versions installed.");
        return Ok(());
    }

    println!("Installed Buildkite agent versions:");
    for version in versions {
        let mut version_str = format!("  {}", version);

        if Some(&version) == current_version.as_ref() {
            version_str = format!("* {}", version_str.trim_start());
        }

        if Some(&version) == config.default_version.as_ref() {
            version_str = format!("{} (default)", version_str);
        }

        println!("{}", version_str);
    }

    Ok(())
}

pub fn list_installed_versions() -> Result<Vec<String>> {
    let versions_file_path = versions_file();
    let file = File::open(versions_file_path)?;
    let reader = BufReader::new(file);
    let versions: Vec<String> = reader
        .lines()
        .filter_map(|line| line.ok())
        .filter(|line| !line.trim().is_empty())
        .collect();
    Ok(versions)
}
