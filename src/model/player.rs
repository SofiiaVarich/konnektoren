use serde::{Deserialize, Serialize};

pub const PLAYER_KEY: &str = "player_profile";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
    /// The player's name
    pub name: String,
    /// The player's solana account
    pub account: Option<String>,
}

impl Default for Player {
    fn default() -> Self {
        Self {
            name: "Anonymous".into(),
            account: None,
        }
    }
}
