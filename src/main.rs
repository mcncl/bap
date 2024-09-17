mod commands;
mod config;
mod internal;
mod utils;

use anyhow::Result;
use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "bap",
    about = "Buildkite Agent Partner - A tool for managing Buildkite agent versions",
    long_about = None
)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// List installed Buildkite agent versions
    List,

    /// List available remote Buildkite agent versions
    ListRemote,

    /// Set the Buildkite agent version for the current directory
    Use(VersionArg),

    /// Download and install a specific Buildkite agent version
    Get(VersionArg),

    /// Set the default Buildkite agent version
    Default(VersionArg),

    /// Run the Buildkite agent
    Run(OptionalVersionArg),

    /// Set the authentication token for a specific Buildkite agent version
    Auth(VersionArg),

    /// Uninstall a specific Buildkite agent version
    Uninstall(VersionArg),
}

#[derive(Args)]
struct VersionArg {
    /// The version of the Buildkite agent
    version: String,
}

#[derive(Args)]
struct OptionalVersionArg {
    /// The version of the Buildkite agent (optional)
    version: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    utils::ensure_bap_directories()?;

    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::List) => {
            commands::list::run()?;
        }
        Some(Commands::ListRemote) => {
            commands::list_remote::run().await?;
        }
        Some(Commands::Use(args)) => {
            commands::use_version::run(&args.version)?;
        }
        Some(Commands::Get(args)) => {
            commands::install::run(&args.version).await?;
        }
        Some(Commands::Default(args)) => {
            commands::default::run(&args.version)?;
        }
        Some(Commands::Run(args)) => {
            commands::run::run(args.version.as_deref()).await?;
        }
        Some(Commands::Auth(args)) => {
            commands::auth::run(&args.version)?;
        }
        Some(Commands::Uninstall(args)) => {
            commands::uninstall::run(&args.version)?;
        }
        None => {
            println!("No command specified. Use --help for usage information.");
        }
    }

    Ok(())
}
