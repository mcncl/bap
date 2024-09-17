use crate::config::Config;
use anyhow::Result;

pub fn run(version: &str) -> Result<()> {
    let mut config = Config::load()?;
    config.default_version = Some(version.to_string());
    config.save()?;
    println!("Default Buildkite agent version set to {}", version);
    Ok(())
}
