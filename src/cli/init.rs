use crate::error::RpassError;
use crate::store::Store;
use crate::utils::git;
use clap::Args;
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
    pub fn run(&self, path_string: &String) -> Result<(), RpassError> {
        let store = Store::init(self.key.to_owned(), PathBuf::from(path_string))?;

        store.save_index()?;

        if self.git {
            git(path_string, vec!["init"])?;
            git(path_string, vec!["add", "."])?;
            git(path_string, vec!["commit", "-m", "'Initialize store'"])?;
        }

        println!(
            "\n{} initialized a new store at {}\n",
            format!("rpass"),
            format!("{}", path_string)
        );

        Ok(())
    }
}
