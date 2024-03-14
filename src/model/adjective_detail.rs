use super::DetailTrait;
use serde::{Deserialize, Serialize};
#[cfg(feature = "openapi")]
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
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
