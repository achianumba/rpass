use std::fs::{create_dir_all, rename};

use clap::Args;
use eyre::{bail, Context, Result};

use crate::{blue, red, store::Store, utils::git, yellow};

/// Move or rename of a secret or group of secrets
#[derive(Debug, Args)]
pub struct Move {
    /// The secret or path to be copied
    #[arg()]
    from: String,

    /// The destination where the secret or group will be copied
    #[arg()]
    to: String,
}

impl Move {
    pub fn run(&self, path_string: &String) -> Result<()> {
        let mut store = Store::load(path_string)?;
        let from = store.get_path(&self.from)?;

        if !from.exists() {
            bail!(red!("Failed to move secret/group. Entry named {} does not exist", &self.from));
        }

        let mut to = store.set_entry_path(&self.to)?;

        if to.exists() {
            let answer = store
                .read_and_echo_user_input(format!(
                    "A secret or group named {} already exists. {}? [y/N]",
                    blue!("{}", &self.to),
                    yellow!("Do you wish to overwrite its contents")
                ))?
                .to_ascii_lowercase();

            if answer != "y".to_string() {
                return Ok(());
            }
        }

        if from.is_file() {
            to.set_extension("gpg");
        }

        store.save_index()?;

        if store.is_repo() {
            git(
                path_string,
                [
                    "mv",
                    from.display().to_string().as_str(),
                    to.display().to_string().as_str(),
                ],
            )?;

            git(path_string, ["commit", "-m", "'move entry'"])?;
        } else {
            if let Some(d) = to.parent() {
                if !d.exists() {
                    create_dir_all(d).wrap_err(red!(
                        "Failed to move {} to {}",
                        &self.from,
                        &self.to
                    ))?;
                }
            }

            rename(from, to).wrap_err(red!("Failed to move {} to {}", &self.from, &self.to))?;
        }

        println!(
            "Moved {} to {}",
            blue!("{}", &self.from),
            blue!("{}", &self.to)
        );

        Ok(())
    }
}
