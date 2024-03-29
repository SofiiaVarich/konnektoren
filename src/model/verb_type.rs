use super::{TestType, TypeTrait};
use serde::{Deserialize, Serialize};
use std::fmt;
use strum::EnumIter;
#[cfg(feature = "openapi")]
use utoipa::ToSchema;

#[derive(Default, Serialize, Deserialize, Debug, PartialEq, Clone, EnumIter)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
pub enum VerbType {
    #[default]
    #[serde(rename = "an")]
    An,
    #[serde(rename = "auf")]
    Auf,
    #[serde(rename = "für")]
    Fuer,
    #[serde(rename = "in")]
    In,
    #[serde(rename = "mit")]
    Mit,
    #[serde(rename = "nach")]
    Nach,
    #[serde(rename = "über")]
    Ueber,
    #[serde(rename = "um")]
    Um,
    #[serde(rename = "unter")]
    Unter,
    #[serde(rename = "von")]
    Von,
    #[serde(rename = "vor")]
    Vor,
    #[serde(rename = "zu")]
    Zu,
}

impl fmt::Display for VerbType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            VerbType::An => "an",
            VerbType::Auf => "auf",
            VerbType::Fuer => "für",
            VerbType::In => "in",
            VerbType::Mit => "mit",
            VerbType::Nach => "nach",
            VerbType::Ueber => "über",
            VerbType::Um => "um",
            VerbType::Unter => "unter",
            VerbType::Von => "von",
            VerbType::Vor => "vor",
            VerbType::Zu => "zu",
        };
        write!(f, "{}", s)
    }
}

impl TypeTrait for VerbType {
    fn get_type() -> String {
        "Verben mit Präpositionen".to_string()
    }

    fn get_t() -> TestType {
        TestType::Verbs
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct CategoryWrapper {
    category: VerbType,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_an() {
        let json = r#"{"category": "an"}"#;
        let category: CategoryWrapper = serde_json::from_str(json).unwrap();
        assert_eq!(category.category, VerbType::An);
    }
}
