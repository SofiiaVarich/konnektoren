use serde::{Deserialize, Serialize};

use super::detail_trait::DetailTrait;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
pub struct KonnektorDetail {
    pub konnektor: String,
    pub example: String,
}

impl DetailTrait for KonnektorDetail {
    fn get_detail(&self) -> String {
        self.konnektor.clone()
    }

    fn get_example(&self) -> String {
        self.example.clone()
    }
}
