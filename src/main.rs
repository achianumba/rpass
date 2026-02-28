use clap::Parser;
use miette::Result;

/// Invokes the CLI.
fn main() -> Result<()> {
    rpass::cli::Cli::parse().run()?;
    Ok(())
}
