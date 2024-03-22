use serde::{Deserialize, Serialize};

use super::DetailTrait;
#[cfg(feature = "openapi")]
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
pub struct NomenDetail {
    pub nomen: String,
    pub example: String,
}

impl DetailTrait for NomenDetail {
    fn get_detail(&self) -> String {
        self.nomen.clone()
    }

    fn get_example(&self) -> String {
        self.example.clone()
    }
}
