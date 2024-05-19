use serde::{Deserialize, Serialize};

use super::DetailTrait;
#[cfg(feature = "openapi")]
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Default)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
pub struct VorgangspassivDetail {
    pub verb: String,
    pub example: String,
}

impl DetailTrait for VorgangspassivDetail {
    fn get_detail(&self) -> String {
        self.verb.clone()
    }

    fn get_example(&self) -> String {
        self.example.clone()
    }
}
