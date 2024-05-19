use super::{AdjectiveType, KonnektorType, NomenType, TypeTrait, VerbType, VorgangspassivType};
use serde::{Deserialize, Serialize};
use strum::EnumIter;

#[cfg(feature = "openapi")]
use utoipa::ToSchema;

#[derive(Default, Debug, Clone, Copy, PartialEq, Serialize, EnumIter, Deserialize, Hash, Eq)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
pub enum TestType {
    #[default]
    Konnektoren,
    Adjectives,
    Verbs,
    Nomen,
    Vorgangspassiv,
}

impl std::fmt::Display for TestType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let s = match self {
            TestType::Konnektoren => KonnektorType::get_type(),
            TestType::Adjectives => AdjectiveType::get_type(),
            TestType::Verbs => VerbType::get_type(),
            TestType::Nomen => NomenType::get_type(),
            TestType::Vorgangspassiv => VorgangspassivType::get_type(),
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
