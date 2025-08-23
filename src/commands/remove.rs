use std::fs::{remove_dir_all, remove_file};

use clap::Args;
use eyre::{Result, WrapErr};

use crate::{blue, red, store::Store, utils::git, yellow};

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
        let mut store = Store::load(path_string)?;
        let entry_file = store.get_path(&self.name)?;

        if entry_file.is_dir() {
            let answer = store
                .read_and_echo_user_input(format!(
                    "{} is a {}. Do you want to remove it anyway? [y/N]",
                    blue!("{}", &self.name),
                    yellow!("directoy containing multiple secrets")
                ))?
                .to_ascii_lowercase();

            if answer != "y".to_string() {
                return Ok(());
            }

            if store.is_repo() {
                git(
                    path_string,
                    ["rm", "-r", "-f", entry_file.as_os_str().to_str().unwrap()],
                )?;

                git(path_string, ["commit", "-m", "'remove group'"])?;
            } else {
                remove_dir_all(&entry_file)
                    .wrap_err(red!("Failed to remove entry group named {}", &self.name))?;
            }
        }

        if entry_file.is_file() {
            let answer = store
                .read_and_echo_user_input(format!(
                    "{} {}? [y/N]",
                    yellow!("Are you sure you want to delete"),
                    blue!("{}", &self.name)
                ))?
                .to_ascii_lowercase();

            if answer != "y".to_string() {
                return Ok(());
            }

            if store.is_repo() {
                git(
                    path_string,
                    ["rm", "-f", entry_file.as_os_str().to_str().unwrap()],
                )?;

                git(path_string, ["commit", "-m", "'remove entry'"])?;
            } else {
                remove_file(entry_file)
                    .wrap_err(red!("Failed to remove entry named {}", &self.name))?;
            }
        }

        println!("Removed {}", blue!("{}", &self.name));

        Ok(())
    }
}
