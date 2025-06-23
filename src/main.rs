use clap::Parser;
use eyre::Result;

mod commands;
use crate::commands::Commands;

fn main() -> Result<()> {
    match RPass::parse().command {
        Commands::Init(init) => init.run()?
    };

    Ok(())
}

/// A password manager for the CLI
#[derive(Debug, Parser)]
#[command(name = "rpass")]
#[clap(author, version, about)]
struct RPass {
    #[command(subcommand)]
    command: Commands,
}
