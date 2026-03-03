#![doc = include_str!("../docs/cli-header.md")]

use clap::Parser;

/// Invokes rpass command.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    rpass::cli::RpassCli::parse().run()?;
    Ok(())
}
