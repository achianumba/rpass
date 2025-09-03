use crate::store::{Store, StoreIndex};
use crate::utils::git;
use crate::{blue, green, red};
use clap::Args;
use miette::{Result, bail, miette};
use std::collections::HashMap;
use std::fs::{create_dir_all, read_dir};
use std::path::PathBuf;

/// Initialize a new password store
#[derive(Debug, Args)]
#[command(alias = "ini")]
pub struct Init {
    ///GPG key ID to use when encrypting password files.
    /// Prompts the user for a symmetric passphrase if no ID is supplied.
    #[arg(short, long, group = "enc")]
    pub key: Option<String>,

    /// Track changes to the store through Git
    #[arg(short, long, required = false)]
    git: bool,
}

impl Init {
    /// Create a new folder at `path_string` and saves `store.toml` to the created folder.
    pub fn run(&self, path_string: &String) -> Result<()> {
        let path = PathBuf::from(path_string);
        let file = path.join("store.toml");

        if path.exists() {
            let paths_count = read_dir(&path)
                .map_err(|e| {
                    miette!(
                        "{} {}",
                        red!("Failed to read the contents of {}", path_string),
                        e.to_string()
                    )
                })?
                .count();

            if paths_count > 1 {
                bail!(red!(
                    "Aborting password store initialization. '{}' already exists and isn't empty.",
                    &path.display()
                ));
            }
        } else {
            let msg = red!("Failed to create password store at '{}'", &path.display());
            create_dir_all(&path).map_err(|e| miette!("{}. {}", msg, e.to_string()))?;
        };

        let store = Store {
            index: StoreIndex {
                key: self.key.to_owned(),
                paths: HashMap::new(),
                name: "rpass::store::index".to_string(),
            },
            path,
            file,
        };

        store.save_index()?;

        if self.git {
            git(path_string, vec!["init"])?;
            git(path_string, vec!["add", "."])?;
            git(path_string, vec!["commit", "-m", "'Initialize store'"])?;
        }

        println!(
            "\n{} initialized a new store at {}\n",
            green!("rpass"),
            blue!("{}", path_string)
        );

        Ok(())
    }
}
