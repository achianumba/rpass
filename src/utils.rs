use std::env::set_current_dir;
use std::ffi::OsStr;
use std::path::PathBuf;
use std::process::Command;

use crate::error::RpassError;

/// Run a given Git command against the store
pub fn git<I: IntoIterator<Item = S>, S: AsRef<OsStr>>(
    path_string: &String,
    args: I,
) -> Result<(), RpassError> {
    let store_path = PathBuf::from(path_string);

    set_current_dir(store_path).map_err(|e| RpassError::Io(e))?;
    // format!(
    //         "{}. {}",
    //         format!("Failed to set working directory to {}", path_string),
    //         e.to_string()
    //     )

    let output = Command::new("git").args(args).output().map_err(
        |e| RpassError::Io(e), // miette!(
                               //     "{}. {}",
                               //     format!("Failed to run git command in {}", path_string),
                               //     e.to_string()
                               // )
    )?;

    if output.status.success() {
        let msg = String::from_utf8(output.stdout).map_err(|e| {
            RpassError::Message(format!(
                "{}. {}",
                "Failed to convert git command's error message to readable text",
                e.to_string()
            ))
        })?;

        println!("{}", msg);
    } else {
        return Err(RpassError::Message(
            String::from_utf8(output.stderr).map_err(|e| RpassError::Message(e.to_string()))?,
        ));
    }

    Ok(())
}
