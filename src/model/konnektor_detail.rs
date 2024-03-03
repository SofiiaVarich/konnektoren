use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub struct KonnektorDetail {
    pub konnektor: String,
    pub example: String,
}
