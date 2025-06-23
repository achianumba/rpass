use crate::store::Store;
use clap::Args;
use eyre::Result;
use std::path::PathBuf;

/// Initialize a new password store
#[derive(Debug, Args)]
#[command(alias = "ini")]
pub struct Init {
    ///GPG key ID to use when encrypting password files.
    /// Prompts the user for a symmetric passphrase if no ID is supplied.
    #[arg(short, long, group = "enc")]
    pub key: Option<String>,
}

impl Init {
    /// Create a new folder at `path_string` and saves `store.toml` to the created folder.
    pub fn run(&self, path_string: &String) -> Result<()> {
        Store::init(self.key.to_owned(), PathBuf::from(path_string))?.save_index()?;

        println!("\n\x1b[1;32mrpass\x1b[0m initialized a new store at '{path_string}'\n");

        Ok(())
    }
}
