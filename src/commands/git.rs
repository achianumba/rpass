use clap::Args;
use eyre::Result;

use crate::utils::git;

/// Execute git commands against the store
#[derive(Debug, Args)]
pub struct Git {
    #[arg(required = true, allow_hyphen_values = true)]
    args: Vec<String>,
}

impl Git {
    pub fn run(&self, path_string: &String) -> Result<()> {
        git(path_string, self.args.to_owned())?;
        Ok(())
    }
}
