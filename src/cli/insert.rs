use std::{collections::HashMap, fs::create_dir_all};

use clap::Args;

use crate::{error::RpassError, store::Store, utils::git};

/// Add a new secret to the store
#[derive(Debug, Args)]
#[command(alias = "ins")]
pub struct Insert {
    /// Allow custom fields (other than username & password)
    #[arg(short, long, required = false)]
    custom_fields: bool,

    #[arg(short, long, required = false)]
    echo: bool,

    /// The secret's name. Paths are separated by UNIX path separator.
    /// E.g. `gmail` becomes `index.gmail` while `gmail/bob` becomes `index.gmail.bob`
    #[arg()]
    name: String,
}

impl Insert {
    pub fn run(&self, path_string: &String) -> Result<(), RpassError> {
        let mut store = Store::load(path_string)?;
        let mut entry_file = store.set_entry_path(&self.name);

        if entry_file.is_dir() {
            return Err(RpassError::Message(format!(
                "Failed to save entry. '{}' is a folder containing at least one other secret.",
                &self.name
            )));
        }

        entry_file.set_extension("gpg");

        if entry_file.exists() {
            return Err(RpassError::Message(format!(
                "The store already contains a secret named '{}'",
                self.name
            )));
        }

        let mut entry: HashMap<String, String> = HashMap::new();

        if !self.custom_fields {
            entry.insert(
                "username".to_string(),
                store.read_user_input("username".to_string(), &true)?,
            );

            let password = store.read_user_input("password".to_string(), &self.echo)?;
            let password_confirmation =
                store.read_user_input("Confirm password".to_string(), &self.echo)?;

            if password != password_confirmation {
                return Err(RpassError::Message(format!("Passwords do not match")));
            }

            entry.insert("password".to_string(), password);
        } else {
            loop {
                println!(
                    "Enter {} as the {} when you're done setting custom fields.",
                    format!("DONE!"),
                    format!("field name")
                );

                let field = store.read_user_input("Field name".to_string(), &true)?;

                if field.trim() == "DONE!" {
                    break;
                }

                entry.insert(
                    field.to_owned(),
                    store.read_user_input(format!("{} value", field), &self.echo)?,
                );
            }
        }

        if let Some(parent_dir) = entry_file.parent() {
            if !parent_dir.exists() {
                create_dir_all(&parent_dir).map_err(|e| {
                    RpassError::Io(e)
                    // format!(
                    //     "{}. {}",
                    //     format!(
                    //         "Failed to create a parent directory for {}",
                    //         parent_dir.display()
                    //     ),
                    //     e.to_string()
                    // )
                })?;
            }
        }

        store.encrypt(format!("{}", entry_file.display()), &self.name, &entry)?;
        store.save_index()?;

        if store.is_repo() {
            git(path_string, ["add", "."])?;
            git(path_string, ["commit", "-m", "'insert secret'"])?;
        }

        println!(
            "Inserted '{}' into the secrets store.",
            format!("{}", self.name)
        );

        Ok(())
    }
}
