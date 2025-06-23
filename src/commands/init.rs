use clap::Args;
use eyre::Result;

/// Initialize a new password store
#[derive(Debug, Args)]
pub struct Init {

}

impl Init {
    pub fn run(&self) -> Result<()> {
        Ok(())
    }
}
