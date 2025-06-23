//! CLI commands.
use clap::Subcommand;
use eyre::Result;
#[cfg(not(debug_assertions))]
use eyre::WrapErr;
#[cfg(not(debug_assertions))]
use std::env::home_dir;
use std::path::MAIN_SEPARATOR_STR;

pub mod init;

use clap::Parser;
use init::Init;

/// A secrets manager for the CLI
#[derive(Debug, Parser)]
#[command(
    name = "rpass",
    author = "Arinze Chianumba",
    version,
    about = "\n\nA pass (UNIX Password Store) inspired secrets manager with asymmetric/symmetric encryption support"
)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Path to the secrets store.
    #[arg(global = true, default_value_t = get_default_store(), env = "DEFAULT_RPASS_STORE")]
    store: String,
}

fn get_default_store() -> String {
    #[cfg(not(debug_assertions))]
    let default_store = home_dir()
        .expect(
            r#"
Failed to determine the current user's home directory.
Pass a specific store path to the --store global option or set 
the 'DEFAULT_RPASS_STORE' environment variable."#,
        )
        .join(".rstore")
        .display()
        .to_string();

    #[cfg(debug_assertions)]
    let default_store = format!(
        "target{}debug{}.rstore",
        MAIN_SEPARATOR_STR, MAIN_SEPARATOR_STR
    );

    default_store
}

/// `rpass` subcommands
#[derive(Debug, Subcommand)]
pub enum Commands {
    Init(Init),
}

impl Cli {
    pub fn run(&self) -> Result<()> {
        match &self.command {
            Commands::Init(init) => init.run(&self.store)?,
        };

        Ok(())
    }
}
