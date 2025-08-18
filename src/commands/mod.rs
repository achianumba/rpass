//! CLI commands.
use clap::Parser;
use clap::Subcommand;
use eyre::{Result, WrapErr};
#[cfg(not(debug_assertions))]
use std::env::home_dir;
use std::path::absolute;
#[cfg(debug_assertions)]
use std::path::MAIN_SEPARATOR_STR;

pub mod init;
pub mod insert;
pub mod list;
pub mod show;

use init::Init;
use insert::Insert;
use list::List;
use show::Show;

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

pub fn get_default_store() -> String {
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
    Insert(Insert),
    List(List),
    Show(Show),
}

impl Cli {
    pub fn run(&mut self) -> Result<()> {
        self.store = absolute(&self.store)
            .wrap_err(format!(
                "Failed to parse absolute path to secrets store at '{}'",
                &self.store
            ))?
            .display()
            .to_string();

        match &self.command {
            Commands::Init(init) => init.run(&self.store)?,
            Commands::Insert(insert) => insert.run(&self.store)?,
            Commands::List(list) => list.run(&self.store)?,
            Commands::Show(show) => show.run(&self.store)?,
        };

        Ok(())
    }
}
