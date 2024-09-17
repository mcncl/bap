use crate::utils::{bin_dir, versions_dir};
use anyhow::{bail, Result};
use flate2::read::GzDecoder;
use reqwest::Client;
use std::fs::{File, OpenOptions};
use std::io::{copy, BufReader, Read, Seek, Write};
use tar::Archive;

pub async fn run(version: &str) -> Result<()> {
    let version_without_v = version.trim_start_matches('v');
    let os = determine_os()?;
    let arch = determine_arch()?;
    let filename = format!(
        "buildkite-agent-{}-{}-{}.tar.gz",
        os, arch, version_without_v
    );
    let url = format!(
        "https://github.com/buildkite/agent/releases/download/v{}/{}",
        version_without_v, filename
    );

    println!("📦 Installing agent {} ({})...", version_without_v, arch);

    let client = Client::new();
    let response = client.get(&url).send().await?;

    if !response.status().is_success() {
        bail!("Failed to download: HTTP status {}", response.status());
    }

    let dest_path = bin_dir().join(version_without_v);
    std::fs::create_dir_all(&dest_path)?;

    let tar_gz_path = dest_path.join(&filename);
    let mut dest = File::create(&tar_gz_path)?;
    let content = response.bytes().await?;
    copy(&mut content.as_ref(), &mut dest)?;

    // Extract the tar.gz file
    let tar_gz = File::open(&tar_gz_path)?;
    let tar = GzDecoder::new(tar_gz);
    let mut archive = Archive::new(tar);
    archive.unpack(&dest_path)?;

    // Remove the tar.gz file after extraction
    std::fs::remove_file(&tar_gz_path)?;

    // Update the list of available versions
    update_versions_list(version_without_v)?;

    println!("🚀 {} installed... ", version_without_v,);
    Ok(())
}

fn determine_arch() -> Result<String> {
    Ok(match std::env::consts::ARCH {
        "x86_64" => "amd64",
        "aarch64" => "arm64",
        arch => bail!("🚫 Unsupported architecture: {}", arch),
    }
    .to_string())
}

fn determine_os() -> Result<String> {
    Ok(match std::env::consts::OS {
        "linux" => "linux",
        "macos" => "darwin",
        os => bail!("Unsupported OS: {}", os),
    }
    .to_string())
}

fn update_versions_list(version: &str) -> Result<()> {
    let versions_file = versions_dir().join("versions");
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(versions_file)?;

    let mut content = String::new();
    BufReader::new(&file).read_to_string(&mut content)?;

    if !content.contains(version) {
        if !content.is_empty() && !content.ends_with('\n') {
            content.push('\n');
        }
        content.push_str(version);
        content.push('\n');

        file.set_len(0)?;
        file.seek(std::io::SeekFrom::Start(0))?;
        file.write_all(content.as_bytes())?;
    }

    Ok(())
}
