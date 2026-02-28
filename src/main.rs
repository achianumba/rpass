use clap::Parser;
use miette::Result;

mod commands;
mod store;
mod utils;

use crate::commands::Cli;

/// Invokes the CLI.
fn main() -> Result<()> {
    Cli::parse().run()?;
    Ok(())
}
