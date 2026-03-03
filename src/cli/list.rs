use std::collections::HashMap;
use std::path::{self, PathBuf};

use clap::{Args, Parser};

use crate::cli::{RpassCli, RpassCliCommand};
use crate::error::RpassError;
use crate::store::{self, Store};

/// List secrets saved in a group or list the fields saved in a group.
#[derive(Debug, Args)]
#[command(alias = "ls")]
pub struct List {
    /// The group/secret to list
    #[arg()]
    pub name: Option<String>,

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
}

impl List {
    /// Creates a [`List`] from the --store argument passed directly to [`super::RpassCli`]
    /// (witout a subcommand) during invocation.
    pub fn from_args(store: &String) -> Result<Self, RpassError> {
        let mut args = vec!["rpass", "list", "--store"];
        args.push(store.as_str());

        let cli = RpassCli::try_parse_from(args).map_err(|e| RpassError::RpassCliCommandError {
            cmd: "list".to_string(),
            source: e,
        })?;

        match cli.command {
            Some(RpassCliCommand::List(list)) => Ok(list),
            _ => Err(clap::Error::new(clap::error::ErrorKind::DisplayHelp)).map_err(|e| {
                RpassError::RpassCliCommandError {
                    cmd: "list".to_string(),
                    source: e,
                }
            })?,
        }
    }

    pub fn run(&self, path_string: &String) -> Result<(), RpassError> {
        let mut store = Store::load(path_string)?;
        let mut name = String::new();
        let mut _root = PathBuf::new();

        if let Some(n) = &self.name {
            name = n.to_owned();
            _root = store.get_path(n)?;
        } else {
            _root = store.path.to_owned();
        }

        if _root.is_file() {
            let fields = store.decrypt(&format!("{}", _root.display()), &name)?;
            println!("\n{} contains the following fields\n", format!("{}", &name));

            for (field, _) in fields {
                println!("- {}", field);
            }

            return Ok(());
        }

        let mut paths: HashMap<String, String> = HashMap::new();

        for (k, v) in store.index.paths.clone() {
            paths.insert(v, k);
        }

        if _root == store.path {
            println!("rPass Store")
        } else {
            let root_name = paths
                .get(&format!("{}", _root.file_name().unwrap().to_str().unwrap()))
                .unwrap();

            println!("{}", format!("{}", root_name));
        }

        store.print_tree(&mut _root, &paths, &"".to_string())?;

        Ok(())
    }
}
