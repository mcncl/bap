use crate::config;
use anyhow::Result;

pub fn run(version: &str) -> Result<()> {
    config::set_local_version(version)?;
    println!(
        "Local Buildkite agent version set to {} for this directory",
        version
    );
    Ok(())
}
