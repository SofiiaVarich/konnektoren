use serde::{Deserialize, Serialize};

use super::detail_trait::DetailTrait;
#[cfg(feature = "openapi")]
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, Default)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
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
