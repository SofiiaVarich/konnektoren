use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct AdjectiveDetail {
    pub adjektiv: String,
    pub example: String,
}
