use crate::utils::bin_dir;
use anyhow::{Context, Result};
use dialoguer::Password;
use std::fs;
use std::path::PathBuf;

pub fn run(version: &str) -> Result<()> {
    let config_path = get_config_path(version)?;

    let token = Password::new()
        .with_prompt("Enter the agent token")
        .interact()?;

    update_config_file(&config_path, &token)?;

    println!("Agent token updated successfully for version {}", version);
    Ok(())
}

fn get_config_path(version: &str) -> Result<PathBuf> {
    let path = bin_dir().join(version).join("buildkite-agent.cfg");
    if !path.exists() {
        anyhow::bail!(
            "Configuration file not found for version {}. Is this version installed?",
            version
        );
    }
    Ok(path)
}

fn update_config_file(path: &PathBuf, token: &str) -> Result<()> {
    let content = fs::read_to_string(path).context("Failed to read configuration file")?;

    let updated_content = content
        .lines()
        .map(|line| {
            if line.trim_start().starts_with("token=") {
                format!("token=\"{}\"", token)
            } else {
                line.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join("\n");

    fs::write(path, updated_content).context("Failed to write updated configuration file")?;

    Ok(())
}
