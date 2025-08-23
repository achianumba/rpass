#[cfg(target_os = "linux")]
use std::time::{Duration, Instant};

use arboard::Clipboard;
#[cfg(target_os = "linux")]
use arboard::SetExtLinux;
use clap::Args;
use eyre::{Result, bail};

use crate::{red, store::Store};

/// Display secrets values and optionally copy them to the clipboard.
#[derive(Debug, Args)]
pub struct Show {
    /// The name of the secret
    #[arg()]
    name: String,

    /// Fields to display or copy to the clipboard
    #[arg(short, long, num_args = 0..)]
    fields: Vec<String>,

    /// Copy the secret to the clipboard
    #[arg(short, long, required = false)]
    clipboard: bool,

    /// The length of time (in seconds) to wait after copying fields to the clipboard.
    #[arg(
        long,
        short,
        env = "DEFAULT_RPASS_WAIT",
        default_value_t = 2,
        requires("clipboard")
    )]
    wait: u64,
}

impl Show {
    pub fn run(&self, path_string: &String) -> Result<()> {
        let store = Store::load(path_string)?;
        let entry_path = store.get_path(&self.name)?;

        if !entry_path.is_file() {
            bail!(red!(
                "'{}' is a group. The store does not contain an entry named '{}'. ",
                &self.name,
                &self.name
            ));
        }

        let entry = store.decrypt(&entry_path, &self.name)?;
        let single_field = self.fields.len() == 1;
        let mut output = if single_field {
            String::new()
        } else {
            String::from("{\n")
        };

        if self.fields.is_empty() {
            for (f, v) in &entry {
                output.push_str(format!("  \"{f}\": \"{v}\",\n").as_str());
            }
        } else if single_field {
            let field = &self.fields[0];

            if !entry.contains_key(field) {
                bail!(red!("'{}' doesn't contain '{}'", &self.name, field));
            }

            output.push_str(entry.get(field).unwrap());
        } else {
            for field in &self.fields {
                if !entry.contains_key(field) {
                    bail!(red!("'{}' doesn't contain '{}'", &self.name, field));
                }

                output.push_str(
                    format!("  \"{field}\": \"{}\",\n", entry.get(field).unwrap()).as_str(),
                );
            }
        }

        if !single_field {
            output.pop().unwrap();
            output.pop().unwrap();
            output.push_str("\n}");
        }

        if self.clipboard {
            println!(
                "Copied the following to the clipboard from {}\n'{}'",
                &self.name, output
            );

            if cfg!(target_os = "linux") {
                let deadline = Instant::now() + Duration::from_secs(self.wait);
                Clipboard::new()?.set().wait_until(deadline).text(&output)?;
            } else {
                Clipboard::new()?.set_text(&output)?;
            }
        } else {
            println!("{output}");
        }

        Ok(())
    }
}
