use serde::{Deserialize, Serialize};

use super::DetailTrait;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
pub struct VerbDetail {
    pub verb: String,
    pub example: String,
}

impl DetailTrait for VerbDetail {
    fn get_detail(&self) -> String {
        self.verb.clone()
    }

    fn get_example(&self) -> String {
        self.example.clone()
    }
}
