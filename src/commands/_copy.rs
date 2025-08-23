use std::fs::{copy, create_dir_all};

use clap::Args;
use eyre::{Context, Result, bail};

use crate::{blue, red, store::Store, utils::git, yellow};

/// Create a copy of a secret
#[derive(Debug, Args)]
pub struct Copy {
    /// The secret to be copied
    #[arg()]
    from: String,

    /// The destination where the secret will be copied
    #[arg()]
    to: String,
}

impl Copy {
    pub fn run(&self, path_string: &String) -> Result<()> {
        let mut store = Store::load(path_string)?;
        let from = store.get_path(&self.from)?;

        if !from.exists() {
            bail!(red!(
                "Failed to copy secret. Entry named {} does not exist",
                &self.from
            ));
        }

        let mut to = store.set_entry_path(&self.to)?;
        let msg = red!("Failed to copy {} to {}", &self.from, &self.to);

        if to.is_dir() {
            bail!(red!(
                "Failed to copy secret to '{}. A group/folder with the same name already exists",
                &self.to
            ))
        }

        to.set_extension("gpg");

        if to.exists() {
            let answer = store
                .read_and_echo_user_input(format!(
                    "A secret named {} already exists. {}? [y/N]",
                    blue!("{}", &self.to),
                    yellow!("Do you wish to overwrite its contents")
                ))?
                .to_ascii_lowercase();

            if answer != "y".to_string() {
                return Ok(());
            }
        } else {
            if let Some(d) = to.parent() {
                if !d.exists() {
                    create_dir_all(d).wrap_err(msg.to_owned())?;
                }
            }
        }

        copy(&from, &to).wrap_err(msg.to_owned())?;

        store.save_index()?;

        if store.is_repo() {
            git(path_string, ["add", to.display().to_string().as_str()])?;
            git(
                path_string,
                [
                    "commit",
                    "-m",
                    format!("'copy {} to {}'", from.display(), to.display()).as_str(),
                ],
            )?;
        }

        println!(
            "Copied {} to {}",
            blue!("{}", &self.from),
            blue!("{}", &self.to)
        );

        Ok(())
    }
}
