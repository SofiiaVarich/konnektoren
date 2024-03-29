use crate::model::NomenType;

use super::{AdjectiveType, KonnektorType, TypeTrait, VerbType};
use serde::{Deserialize, Serialize};
use strum::EnumIter;

#[cfg(feature = "openapi")]
use utoipa::ToSchema;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, EnumIter, Deserialize)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
pub enum TestType {
    Konnektoren,
    Adjectives,
    Verbs,
    Nomen,
}

impl std::default::Default for TestType {
    fn default() -> Self {
        TestType::Konnektoren
    }
}

impl std::fmt::Display for TestType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s = match self {
            TestType::Konnektoren => KonnektorType::get_type(),
            TestType::Adjectives => AdjectiveType::get_type(),
            TestType::Verbs => VerbType::get_type(),
            TestType::Nomen => NomenType::get_type(),
        };
        write!(f, "{}", s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type_default() {
        let test_type: TestType = Default::default();
        assert_eq!(test_type, TestType::Konnektoren);
    }
}
