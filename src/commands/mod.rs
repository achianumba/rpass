use clap::Subcommand;

mod init;
use init::Init;

#[derive(Debug, Subcommand)]
pub enum Commands {
    Init(Init),
}
