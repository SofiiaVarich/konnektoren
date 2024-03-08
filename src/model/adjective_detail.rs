use serde::{Deserialize, Serialize};

use super::DetailTrait;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
pub struct AdjectiveDetail {
    pub adjektiv: String,
    pub example: String,
}

impl DetailTrait for AdjectiveDetail {
    fn get_detail(&self) -> String {
        self.adjektiv.clone()
    }

    fn get_example(&self) -> String {
        self.example.clone()
    }
}
