use super::TypeTrait;
use serde::{Deserialize, Serialize};
use std::fmt;
use strum::EnumIter;
#[cfg(feature = "openapi")]
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, EnumIter)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
pub enum AdjectiveType {
    #[serde(rename = "an")]
    An,
    #[serde(rename = "auf")]
    Auf,
    #[serde(rename = "bei")]
    Bei,
    #[serde(rename = "für")]
    Fuer,
    #[serde(rename = "mit")]
    Mit,
    #[serde(rename = "über")]
    Ueber,
    #[serde(rename = "von")]
    Von,
    #[serde(rename = "zu")]
    Zu,
}

impl Default for AdjectiveType {
    fn default() -> Self {
        AdjectiveType::An
    }
}

impl fmt::Display for AdjectiveType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            AdjectiveType::An => "an",
            AdjectiveType::Auf => "auf",
            AdjectiveType::Bei => "bei",
            AdjectiveType::Fuer => "für",
            AdjectiveType::Mit => "mit",
            AdjectiveType::Ueber => "über",
            AdjectiveType::Von => "von",
            AdjectiveType::Zu => "zu",
        };
        write!(f, "{}", s)
    }
}

impl TypeTrait for AdjectiveType {
    fn get_type() -> String {
        "Adjektive mit Präpositionen".to_string()
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct CategoryWrapper {
    category: AdjectiveType,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_an() {
        let json = r#"{"category": "an"}"#;
        let wrapper: CategoryWrapper = serde_json::from_str(json).unwrap();
        assert_eq!(wrapper.category, AdjectiveType::An);
    }
}
