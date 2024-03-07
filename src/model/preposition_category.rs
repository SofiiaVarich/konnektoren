use super::{adjective_detail::AdjectiveDetail, PrepositionType};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct PrepositionCategory {
    pub category: PrepositionType,
    pub details: Vec<AdjectiveDetail>,
}
