use clap::Args;
use eyre::{bail, Result};

use crate::store::Store;

/// Modify field names and values or
/// add fields to a secret
#[derive(Debug, Args)]
#[command(alias = "update")]
pub struct Edit {
    /// Field names to modify
    #[arg(short, long, num_args = 0..)]
    fields: Vec<String>,

    /// Names of fields whose values should be modified
    #[arg(short, long, num_args = 0..)]
    values: Vec<String>,

    /// New fields to be added to the secret.
    #[arg(short, long, num_args = 0..)]
    new: Vec<String>,

    /// Display Current and new field values (as they're entered/updated)
    #[arg(short, long, required = false)]
    echo: bool,

    /// The secret's name. Paths are separated by UNIX path separator.
    #[arg()]
    name: String,
}

impl Edit {
    pub fn run(&self, path_string: &String) -> Result<()> {
        let mut store = Store::load(path_string)?;
        let entry_file = store.get_path(&self.name)?;

        if entry_file.is_dir() {
            bail!(
                "Failed to edit entry. '{}' is a folder containing at least one other secret.",
                &self.name
            );
        }

        let mut entry = store.decrypt(&entry_file, &self.name)?;

        for field in &self.new {
            let value = store.read_user_input(field.to_owned(), &self.echo)?;

            entry.insert(field.to_owned(), value);
        }

        for field in &self.fields {
            match entry.remove(field) {
                Some(value) => {
                    if self.echo {
                        println!("Current name: {}", &field);
                    }

                    let new_field = store.read_user_input(format!("New {field}"), &self.echo)?;

                    entry.insert(new_field, value.to_owned());
                }
                None => {
                    bail!("'{}' doesn't contain a field named '{}'", &self.name, field);
                }
            };
        }

        for field in &self.values {
            match entry.get(field) {
                Some(value) => {
                    if self.echo {
                        println!("Current value: {}", value);
                    }

                    let new_value =
                        store.read_user_input(format!("New value of {field}"), &self.echo)?;

                    entry.insert(field.to_owned(), new_value);
                }
                None => {
                    bail!("'{}' doesn't contain a field named '{}'", &self.name, field);
                }
            };
        }

        store.save(entry_file, &self.name, &entry)?;

        Ok(())
    }
}
