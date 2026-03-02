//! [`thiserror`](https://docs.rs/thiserror/latest/thiserror) based Rpass custom errors.

use std::io;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum RpassError {
    #[error("Error: {0}")]
    Message(String),
    #[error("I/O Error")]
    Io(#[from] io::Error)
}
