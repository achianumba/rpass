#[cfg(target_os = "linux")]
use std::time::{Duration, Instant};

use arboard::Clipboard;
#[cfg(target_os = "linux")]
use arboard::SetExtLinux;
use clap::Args;
use miette::{Result, bail, miette};

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
        let mut store = Store::load(path_string)?;
        let entry_path = store.get_path(&self.name)?;

        if !entry_path.is_file() {
            bail!(red!(
                "'{}' is a group. The store does not contain an entry named '{}'. ",
                &self.name,
                &self.name
            ));
        }

        let entry = store.decrypt(&entry_path.display().to_string(), &self.name)?;
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

            #[cfg(target_os = "linux")]
            let deadline = Instant::now() + Duration::from_secs(self.wait);

            #[cfg(target_os = "linux")]
            Clipboard::new()
                .map_err(|e| {
                    miette!(
                        "{}. {}",
                        red!("Failed to access to clipboard"),
                        e.to_string()
                    )
                })?
                .set()
                .wait_until(deadline)
                .text(&output)
                .map_err(|e| {
                    miette!(
                        "{}. {}",
                        red!("Failed to access the clipboard"),
                        e.to_string()
                    )
                })?;

            #[cfg(not(target_os = "linux"))]
            Clipboard::new()
                .map_err(|e| {
                    miette!(
                        "{}. {}",
                        red!("Failed to copy entry fields(s) to the clipboard"),
                        e.to_string()
                    )
                })?
                .set_text(&output)
                .map_err(|e| {
                    miette!(
                        "{}. {}",
                        red!("Failed to copy entry fields(s) to the clipboard"),
                        e.to_string()
                    )
                })?;
        } else {
            println!("{output}");
        }

        Ok(())
    }
}
