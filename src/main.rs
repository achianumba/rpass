//! `rpass` is a [`pass`](https://www.passwordstore.org/) inspired secrets manager.
//!
//! > ⚠️ THIS CURRENTLY PROJECT IS A WORK-IN-PROGRESS. SUBSEQUENT RELEASES MAY INCLUDE BREAKING CHANGES
//!
//! ## Features
//!
//! - Asymmetrically encrypted secrets
//! - Symmetrically encrypted secrets
//!
//! ## Installation
//!
//! **Requirements**
//!
//! `rpass` depends on the [`gpgme`](https://crates.io/crates/gpgme) crate for
//! all interactions with a host's GPG keyring. So, it you need to first install
//! the following dependencies.
//!
//! - **Debian/Ubuntu based systems:** `libgpgme11-dev`
//! - **RHEL/Fedora based systems:** `gpgme-devel`
//! - **NixOS:** TODO!
//! - **Alpine:** TODO!
//! - **Arch:** TODO!
//! - **macOS:** `gnupg`
//! - **Windows:** [`Gpg4win`](https://www.gpg4win.org)
//!
//! **Install from source**
//!
//! ```
//! git clone https://github.com/achianumba/rpass.git
//! cd rpass
//! cargo build --release
//! ```
//!
//! **Install from Crates.io**
//!
//! ```
//! cargo binstall rpass
//! ```
use clap::Parser;
use eyre::Result;

mod commands;
mod store;
mod utils;

use crate::commands::Cli;

/// Invokes the CLI.
fn main() -> Result<()> {
    Cli::parse().run()?;
    Ok(())
}
