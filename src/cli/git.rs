use clap::Args;

use crate::{error::RpassError, utils::git};

/// Execute git commands against the store
#[derive(Debug, Args)]
pub struct Git {
    #[arg(required = true, allow_hyphen_values = true)]
    args: Vec<String>,
}

impl Git {
    pub fn run(&self, path_string: &String) -> Result<(), RpassError> {
        git(path_string, self.args.to_owned())?;
        Ok(())
    }
}
