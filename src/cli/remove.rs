use std::fs::{remove_dir_all, remove_file};

use clap::Args;

use crate::{error::RpassError, store::Store, utils::git};

/// Delete a secret from the store
#[derive(Debug, Args)]
#[command(alias = "rm")]
pub struct Remove {
    /// The secret's name. Paths are separated by UNIX path separator.
    #[arg()]
    name: String,
}

impl Remove {
    pub fn run(&self, path_string: &String) -> Result<(), RpassError> {
        let mut store = Store::load(path_string)?;
        let entry_file = store.get_path(&self.name)?;

        if entry_file.is_dir() {
            let answer = store
                .read_and_echo_user_input(format!(
                    "{} is a {}. Do you want to remove it anyway? [y/N]",
                    format!("{}", &self.name),
                    format!("directoy containing multiple secrets")
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
                remove_dir_all(&entry_file).map_err(|e| {
                    RpassError::Io(e)
                    // miette!(
                    //     "{}. {}",
                    //     format!("Failed to remove entry group named {}", &self.name),
                    //     e.to_string()
                    // )
                })?;
            }
        }

        if entry_file.is_file() {
            let answer = store
                .read_and_echo_user_input(format!(
                    "{} {}? [y/N]",
                    format!("Are you sure you want to delete"),
                    format!("{}", &self.name)
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
                remove_file(entry_file).map_err(|e| {
                    RpassError::Io(e)
                    // miette!(
                    //     "{}. {}",
                    //     format!("Failed to remove entry named {}", &self.name),
                    //     e.to_string()
                    // )
                })?;
            }
        }

        println!("Removed {}", format!("{}", &self.name));

        Ok(())
    }
}
