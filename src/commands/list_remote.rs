use crate::commands::{default, install, use_version};
use crate::internal::api::GitHubAPI;
use crate::utils::versions_dir;
use anyhow::Result;
use dialoguer::{theme::ColorfulTheme, Confirm, Select};
use std::collections::HashSet;
use std::fs;

const PAGE_SIZE: usize = 10;

pub async fn run() -> Result<()> {
    let version = select_version().await?;
    handle_selected_version(&version).await?;
    Ok(())
}

pub async fn select_version() -> Result<String> {
    let api = GitHubAPI::new()?;
    let all_releases = api.get_all_releases().await?;

    let versions: Vec<String> = all_releases.into_iter().map(|r| r.tag_name).collect();

    if versions.is_empty() {
        anyhow::bail!("No remote Buildkite agent versions found.");
    }

    let installed_versions = get_installed_versions()?;

    let mut page = 0;
    let total_pages = (versions.len() + PAGE_SIZE - 1) / PAGE_SIZE;

    println!("Select a remote Buildkite agent version");

    loop {
        let start = page * PAGE_SIZE;
        let end = (start + PAGE_SIZE).min(versions.len());
        let current_page_versions = &versions[start..end];

        let mut items: Vec<String> = current_page_versions
            .iter()
            .map(|v| {
                if installed_versions.contains(v) {
                    format!("{} (installed)", v)
                } else {
                    v.clone()
                }
            })
            .collect();

        let mut nav_options = Vec::new();

        if page < total_pages - 1 {
            nav_options.push("Next page");
        }
        if page > 0 {
            nav_options.push("Previous page");
        }
        items.extend(nav_options.iter().map(|&s| s.to_string()));

        let selection = Select::with_theme(&ColorfulTheme::default())
            .items(&items)
            .default(0)
            .interact()?;

        if selection >= current_page_versions.len() {
            let nav_selection = &items[selection];
            if nav_selection == "Next page" {
                page += 1;
            } else if nav_selection == "Previous page" {
                page -= 1;
            }
        } else {
            return Ok(current_page_versions[selection].clone());
        }
    }
}

fn is_version_installed(version: &str) -> Result<bool> {
    let installed_versions = get_installed_versions()?;
    let version_without_v = version.trim_start_matches('v');
    Ok(installed_versions.contains(version_without_v))
}

fn get_installed_versions() -> Result<HashSet<String>> {
    let versions_file = versions_dir().join("versions");
    let content = fs::read_to_string(versions_file)?;
    Ok(content
        .lines()
        .map(|line| line.trim().to_string())
        .collect())
}

async fn handle_selected_version(version: &str) -> Result<()> {
    println!("You selected: {}", version);

    let version_without_v = version.trim_start_matches('v');
    if is_version_installed(version_without_v)? {
        handle_installed_version(version_without_v)?;
    } else {
        handle_not_installed_version(version).await?;
    }

    Ok(())
}

fn handle_installed_version(version: &str) -> Result<()> {
    println!("This version is already installed.");
    let options = vec![
        "Set as local default",
        "Set as global default",
        "Do nothing",
    ];
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("What would you like to do?")
        .items(&options)
        .default(0)
        .interact()?;

    match selection {
        0 => use_version::run(version)?,
        1 => default::run(version)?,
        2 => println!("No action taken."),
        _ => unreachable!(),
    }

    Ok(())
}

async fn handle_not_installed_version(version: &str) -> Result<()> {
    if Confirm::new()
        .with_prompt(format!(
            "Version {} is not installed. Would you like to install it?",
            version
        ))
        .default(true)
        .interact()?
    {
        install::run(version).await?;
        println!("Version {} has been installed.", version);

        if Confirm::new()
            .with_prompt("Would you like to set this as the local default version?")
            .default(true)
            .interact()?
        {
            use_version::run(version)?;
        }
    } else {
        println!("No action taken.");
    }

    Ok(())
}
