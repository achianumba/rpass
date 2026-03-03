//! [`clap`] based Rpass command line interface.

use clap::{ArgAction, ArgGroup, Parser, Subcommand};

use crate::{error::RpassError, store};

#[derive(Debug, Parser)]
#[command(about, version)]
#[command(name = "rpass")]
#[command(max_term_width = 60)]
#[command(author = "Arinze Chianumba")]
#[command(before_long_help = include_str!("../docs/cli-header.md"))]
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
    ///
    /// --path may also be passed instead of --store
    #[arg(
        short,
        long,
        visible_alias = "path",
        default_value_t = store::RpassStore::default_store_string(),
        env = "RPASS_STORE"
    )]
    pub store: String,

    /// Run without writing any changes to the store.
    #[arg(short, long, required = false, global = true)]
    pub dry_run: bool,

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
    /// Initialize a new secrets store
    #[command(alias = "ini")]
    Init {
        /// Track changes to the store through Git
        #[arg(short, long, required = false)]
        git: bool,

        /// Pub PGG key ID to use when encrypting password files.
        /// Prompts the user for a symmetric passphrase if no ID is supplied.
        #[arg(short, long, group = "enc", num_args = 0.., alias = "gpg-id")]
        keys: Vec<String>,
    },
    /// Add a secret to the store
    #[command(alias = "add")]
    Insert {
        /// Allow custom fields (other than username & password)
        #[arg(short, long, required = false)]
        custom_fields: bool,

        #[arg(short, long, required = false)]
        echo: bool,

        /// The secret's name. Paths are separated by UNIX path separator.
        /// E.g. `gmail` becomes `index.gmail` while `gmail/bob` becomes `index.gmail.bob`
        #[arg()]
        name: String,
    },
    /// List secrets saved in a group or list the fields saved in a group.
    #[command(alias = "ls")]
    List {
        /// The group/secret to list
        #[arg()]
        name: Option<String>,

        /// Path to the secrets store to list groups and secret fields from.
        ///
        /// Rpass looks for the RPASS_STORE environment variable and falls back
        /// to PASSWORD_STORE_DIR for pass compatibility if when running non-init commands,
        /// the default store does not exist.
        #[arg(
        short,
        long,
        visible_alias = "path",
        global = true,
        default_value_t = format!("{}", store::RpassStore::default_store().expect("Failed to determine user's default store").display()),
        env = "RPASS_STORE"
    )]
        store: String,
    },
    /// Display secrets values and optionally copy them to the clipboard.
    Show {
        /// The name of the secret
        #[arg()]
        name: String,

        /// Fields to display or copy to the clipboard
        #[arg(short, long, num_args = 0..)]
        fields: Vec<String>,

        /// Copy the secret to the clipboard
        #[arg(short, long, required = false)]
        clipboard: bool,

        /// The length of time (in seconds) to wait after copying fields to the clipboard.
        #[arg(
            long,
            short,
            env = "DEFAULT_RPASS_WAIT",
            default_value_t = 2,
            requires("clipboard")
        )]
        wait: u64,
    },
    /// Modify field names and values or
    /// add fields to a secret
    #[command(alias = "update")]
    Edit {
        /// Field names to modify
        #[arg(short, long, num_args = 0..)]
        fields: Vec<String>,

        /// Names of fields whose values should be modified
        #[arg(short, long, num_args = 0..)]
        values: Vec<String>,

        /// New fields to be added to the secret.
        #[arg(short, long, num_args = 0..)]
        new: Vec<String>,

        /// Display Current and new field values (as they're entered/updated)
        #[arg(short, long, required = false)]
        echo: bool,

        /// The secret's name. Paths are separated by UNIX path separator.
        #[arg()]
        name: String,
    },
    /// Delete a secret from the store
    #[command(alias = "rm")]
    Remove {
        /// The secret's name. Paths are separated by UNIX path separator.
        #[arg()]
        name: String,
    },
    /// Generate and optionally insert a secret's password or passphrase
    #[command(alias = "gen", group(ArgGroup::new("_type").required(true)))]
    Generate {
        /// Generated password/passphrase character/word count.
        /// [default: 32-character password or 6-word passphrase]
        #[arg(short, long)]
        length: Option<u8>,

        /// Generate a password
        #[arg(group = "_type", short, long)]
        password: bool,

        /// Omit special/numeric characters or letters from the generated password
        #[arg(
      short,
      long,
      value_parser = ["letter", "special", "number"],
      num_args = 1..=2,
      action = ArgAction::Set,
      required_if_eq("password", ""),
      conflicts_with = "passphrase")
      ]
        omit: Vec<String>,

        /// Generate a passphrase.
        #[arg(group = "_type", short = 'P', long)]
        passphrase: bool,

        /// The secret's name
        #[arg()]
        name: Option<String>,
    },
    /// Execute git commands against the store
    Git {
        #[arg(required = true, allow_hyphen_values = true)]
        args: Vec<String>,
    },
    /// Create a copy of a secret
    #[command(alias = "cp")]
    Copy {
        /// The secret to be copied
        #[arg()]
        from: String,

        /// The destination where the secret will be copied
        #[arg()]
        to: String,
    },
    /// Move or rename of a secret or group of secrets
    #[command(alias = "mv")]
    Move {
        /// The secret or path to be copied
        #[arg()]
        from: String,

        /// The destination where the secret or group will be copied
        #[arg()]
        to: String,
    }
}

impl RpassCli {
    /// Run the invoked [`RpassCliCommand`]
    pub fn run(self) -> Result<(), RpassError> {
        store::RpassStore::from_cli(self)?;
        Ok(())
    }
}
