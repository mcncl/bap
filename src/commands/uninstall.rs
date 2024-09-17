use crate::config::Config;
use crate::utils::{bin_dir, versions_dir};
use anyhow::{Context, Result};
use std::fs;
use std::io::{BufReader, Read, Seek, Write};

pub fn run(version: &str) -> Result<()> {
    // Check if the version is installed
    // That could be a silly error otherwise
    let version_dir = bin_dir().join(version);
    if !version_dir.exists() {
        anyhow::bail!("Version {} is not installed.", version);
    }

    // Remove the version directory
    // This seems like the "simplest" way
    fs::remove_dir_all(&version_dir).context(format!(
        "Failed to remove directory for version {}",
        version
    ))?;

    // Remove the version from the versions file
    // This could just cause confusion, so best to do the cleanup
    remove_from_versions_file(version)?;

    // Check if it's the default version and remove if so
    remove_from_default_if_needed(version)?;

    println!(
        "Successfully uninstalled Buildkite agent version {}",
        version
    );
    Ok(())
}

fn remove_from_versions_file(version: &str) -> Result<()> {
    let versions_file = versions_dir().join("versions");
    let mut file = fs::OpenOptions::new()
        .read(true)
        .write(true)
        .open(&versions_file)
        .context("Failed to open versions file")?;

    let mut content = String::new();
    BufReader::new(&file).read_to_string(&mut content)?;

    let updated_content: Vec<_> = content
        .lines()
        .filter(|&line| line.trim() != version)
        .collect();

    file.set_len(0)?;
    file.seek(std::io::SeekFrom::Start(0))?;
    writeln!(file, "{}", updated_content.join("\n").trim())?;

    Ok(())
}

fn remove_from_default_if_needed(version: &str) -> Result<()> {
    let mut config = Config::load()?;
    if config.default_version.as_deref() == Some(version) {
        config.default_version = None;
        config.save()?;
        println!("Removed {} as the default version.", version);
    }
    Ok(())
}
