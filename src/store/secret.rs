use serde::{Deserialize, Serialize};

/// Secret variants that can be saved to the store
#[derive(Debug, Deserialize, Serialize)]
pub enum RpassSecret {
    UsernameAndPassword,
    EmailAndPassword,
    Custom,
}
