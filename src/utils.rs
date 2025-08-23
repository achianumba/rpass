use std::env::set_current_dir;
use std::ffi::OsStr;
use std::path::PathBuf;
use std::process::Command;

use eyre::{Context, Result, bail};

#[macro_export]
macro_rules! red {
    ($($args:tt)*) => {
        format!("\x1b[1;31m{}\x1b[0m", format!($($args)*))
    };
}

#[macro_export]
macro_rules! green {
    ($($args:tt)*) => {
        format!("\x1b[1;32m{}\x1b[0m", format!($($args)*))
    };
}

#[macro_export]
macro_rules! yellow {
    ($($args:tt)*) => {
        format!("\x1b[1;33m{}\x1b[0m", format!($($args)*))
    };
}

#[macro_export]
macro_rules! blue {
    ($($args:tt)*) => {
        format!("\x1b[1;34m{}\x1b[0m", format!($($args)*))
    };
}

#[macro_export]
macro_rules! purple {
    ($($args:tt)*) => {
        format!("\x1b[1;35m{}\x1b[0m", format!($($args)*))
    };
}

/// Run a given Git command against the store
pub fn git<I: IntoIterator<Item = S>, S: AsRef<OsStr>>(
    path_string: &String,
    args: I,
) -> Result<()> {
    let store_path = PathBuf::from(path_string);

    set_current_dir(store_path)
        .wrap_err(red!("Failed to set working directory to {}", path_string))?;

    let output = Command::new("git")
        .args(args)
        .output()
        .wrap_err(red!("Failed to run git command"))?;

    if output.status.success() {
        println!("{}", String::from_utf8(output.stdout)?);
    } else {
        bail!(String::from_utf8(output.stderr)?)
    }

    Ok(())
}
