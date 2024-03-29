use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    /// The player's name
    name: String,
    /// The player's solana account
    account: String,
}
