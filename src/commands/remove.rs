use clap::Args;
use eyre::{bail, Result};

use crate::store::Store;

/// Delete a secret from the store
#[derive(Debug, Args)]
#[command(alias = "rm")]
pub struct Remove {
    /// The secret's name. Paths are separated by UNIX path separator.
    #[arg()]
    name: String,
}

impl Remove {
    pub fn run(&self, path_string: &String) -> Result<()> {
        let store = Store::load(path_string)?;
        let entry_file = store.get_path(&self.name)?;

        if entry_file.is_dir() {
            bail!(
                "Failed to edit entry. '{}' is a folder containing at least one other secret.",
                &self.name
            );
        }

        store.delete(entry_file, &self.name)?;

        Ok(())
    }
}
