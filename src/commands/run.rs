use crate::commands::install;
use crate::config;
use crate::utils::bin_dir;
use anyhow::{Context, Result};
use dialoguer::Confirm;
use std::path::PathBuf;
use std::process::Stdio;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command as AsyncCommand;
use tokio::signal::ctrl_c;

pub async fn run(specified_version: Option<&str>) -> Result<()> {
    let version = match specified_version {
        Some(v) => v.to_string(),
        None => get_version().await?,
    };

    let agent_path = ensure_version_installed(&version).await?;

    println!("Running Buildkite agent version {}...", version);

    let mut child = AsyncCommand::new(agent_path)
        .arg("start")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .context("Failed to start buildkite-agent")?;

    let stdout = child.stdout.take().expect("Failed to capture stdout");
    let stderr = child.stderr.take().expect("Failed to capture stderr");

    let mut stdout_reader = BufReader::new(stdout).lines();
    let mut stderr_reader = BufReader::new(stderr).lines();

    let output_handler = tokio::spawn(async move {
        loop {
            tokio::select! {
                result = stdout_reader.next_line() => {
                    match result {
                        Ok(Some(line)) => println!("{}", line),
                        Ok(None) => break,
                        Err(e) => eprintln!("Error reading stdout: {}", e),
                    }
                }
                result = stderr_reader.next_line() => {
                    match result {
                        Ok(Some(line)) => eprintln!("{}", line),
                        Ok(None) => break,
                        Err(e) => eprintln!("Error reading stderr: {}", e),
                    }
                }
            }
        }
    });

    tokio::select! {
        _ = ctrl_c() => {
            println!("");
            child.kill().await.context("Failed to kill buildkite-agent process")?;
        }
        status = child.wait() => {
            if let Err(e) = status {
                eprintln!("Buildkite agent process error: {}", e);
            }
        }
    }

    output_handler.await.context("Failed to handle output")?;

    Ok(())
}

async fn get_version() -> Result<String> {
    if let Some(version) = config::get_version()? {
        return Ok(version);
    }

    let versions = crate::commands::list::list_installed_versions()?;
    if versions.is_empty() {
        println!("No Buildkite agent versions installed.");
        let install = Confirm::new()
            .with_prompt("Do you want to install a version?")
            .default(true)
            .interact()?;

        if install {
            let version = crate::commands::list_remote::select_version().await?;
            install::run(&version).await?;
            return Ok(version);
        } else {
            anyhow::bail!("Cannot run Buildkite agent: no version installed or selected.");
        }
    }

    let selection = dialoguer::Select::new()
        .with_prompt("Select a Buildkite agent version to run")
        .items(&versions)
        .interact()?;

    Ok(versions[selection].clone())
}

async fn ensure_version_installed(version: &str) -> Result<PathBuf> {
    let agent_path = bin_dir().join(version).join("buildkite-agent");

    if !agent_path.exists() {
        println!("Buildkite agent version {} is not installed.", version);
        let install = Confirm::new()
            .with_prompt(format!("Do you want to install version {}?", version))
            .default(true)
            .interact()?;

        if install {
            install::run(version).await?;
        } else {
            anyhow::bail!(
                "Cannot run Buildkite agent: version {} is not installed.",
                version
            );
        }
    }

    Ok(agent_path)
}
