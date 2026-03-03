//! [`clap`] based Rpass command line interface.

use clap::{Parser, Subcommand};
#[cfg(debug_assertions)]
use std::path::MAIN_SEPARATOR_STR;

pub mod _copy;
pub mod _move;
pub mod edit;
pub mod generate;
pub mod git;
pub mod init;
pub mod insert;
pub mod list;
pub mod remove;
pub mod show;

use crate::{error::RpassError, store};
use _copy::Copy;
use _move::Move;
use edit::Edit;
use generate::Generate;
use git::Git;
use init::Init;
use insert::Insert;
use list::List;
use remove::Remove;
use show::Show;

#[derive(Debug, Parser)]
#[command(about, version)]
#[command(name = "rpass")]
#[command(max_term_width = 60)]
#[command(author = "Arinze Chianumba")]
#[command(before_long_help = include_str!("../../docs/cli-header.md"))]
#[command(args_conflicts_with_subcommands = true)]
pub struct RpassCli {
    /// Available [`RpassCli`] subcommands.
    #[command(subcommand)]
    pub command: Option<RpassCliCommand>,

    /// Path to the secrets store to use.
    ///
    /// Rpass looks for the RPASS_STORE environment variable and falls back
    /// to PASSWORD_STORE_DIR for pass compatibility if when running non-init commands,
    /// the default store does not exist.
    #[arg(
        short,
        long,
        visible_alias = "path",
        global = true,
        default_value_t = store::RpassStore::default_store_string(),
        env = "RPASS_STORE"
    )]
    pub store: String,

    /// Arguments and options passed directly to [`RpassCli`] when invoked without an [`RpassCliCommand`].
    ///
    /// Invoking [`RpassCli`] without a subcommand behaves the same as "rpass list" or "rpass show"
    /// depending on whether 0 or more than 0 [`RpassCli::args`] are passed are passed directly to the invocation.
    #[arg()]
    pub args: Vec<String>,
}

/// Available [`RpassCli`] subcommands
#[derive(Debug, Subcommand)]
pub enum RpassCliCommand {
    Init(Init),
    Insert(Insert),
    List(List),
    Show(Show),
    Edit(Edit),
    Remove(Remove),
    Generate(Generate),
    Git(Git),
    Copy(Copy),
    Move(Move),
}

impl RpassCli {
    /// Run the invoked [`RpassCliCommand`]
    pub fn run(self) -> Result<(), RpassError> {
        store::RpassStore::from_cli(self)?;
        Ok(())
    }
}
