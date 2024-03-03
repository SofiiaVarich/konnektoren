use super::konnektor_detail::KonnektorDetail;
use super::konnektor_type::KonnektorType;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct KonnektorCategory {
    pub category: KonnektorType,
    pub details: Vec<KonnektorDetail>,
}
