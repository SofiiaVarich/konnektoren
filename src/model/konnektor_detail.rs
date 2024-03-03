use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct KonnektorDetail {
    konnektor: String,
    example: String,
}
