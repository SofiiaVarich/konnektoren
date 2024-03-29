use super::{TestType, TypeTrait};
use serde::{Deserialize, Serialize};
use std::fmt;
use strum::EnumIter;
#[cfg(feature = "openapi")]
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, EnumIter)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
pub enum NomenType {
    #[serde(rename = "bringen")]
    Bringen,
    #[serde(rename = "geraten")]
    Geraten,
    #[serde(rename = "kommen")]
    Kommen,
    #[serde(rename = "nehmen")]
    Nehmen,
    #[serde(rename = "setzen")]
    Setzen,
    #[serde(rename = "stehen")]
    Stehen,
    #[serde(rename = "stellen")]
    Stellen,
    #[serde(rename = "stoßen")]
    Stossen,
    #[serde(rename = "ziehen")]
    Ziehen,
}

impl Default for NomenType {
    fn default() -> Self {
        NomenType::Bringen
    }
}

impl fmt::Display for NomenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            NomenType::Bringen => "bringen",
            NomenType::Geraten => "geraten",
            NomenType::Kommen => "kommen",
            NomenType::Nehmen => "nehmen",
            NomenType::Setzen => "setzen",
            NomenType::Stehen => "stehen",
            NomenType::Stellen => "stellen",
            NomenType::Stossen => "stoßen",
            NomenType::Ziehen => "ziehen",
        };
        write!(f, "{}", s)
    }
}

impl TypeTrait for NomenType {
    fn get_type() -> String {
        "Nomen-Verb-Verbindungen".to_string()
    }

    fn get_t() -> TestType {
        TestType::Nomen
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct CategoryWrapper {
    category: NomenType,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_an() {
        let json = r#"{"category": "bringen"}"#;
        let category: CategoryWrapper = serde_json::from_str(json).unwrap();
        assert_eq!(category.category, NomenType::Bringen);
    }
}
