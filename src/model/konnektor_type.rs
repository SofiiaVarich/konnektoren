use serde::{Deserialize, Serialize};
use std::fmt;
use strum::EnumIter;

use super::TypeTrait;
#[cfg(feature = "openapi")]
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, EnumIter)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
pub enum KonnektorType {
    #[serde(rename = "Konnektoren mit Nebensatz (Verb am Ende) (= Subjunktionen)")]
    Subjunktionen,
    #[serde(rename = "Konnektoren mit Hauptsatz (Position 0) (= Konjunktionen)")]
    Konjunktionen,
    #[serde(rename = "Konnektoren mit Hauptsatz (Position 1) (= Konjunktionaladverbien)")]
    Konjunktionaladverbien,
    #[serde(rename = "Konnektoren mit Infinitivgruppe")]
    Infinitivgruppe,
    #[serde(rename = "Konnektoren mit besonderer Position")]
    BesonderePosition,
}

impl Default for KonnektorType {
    fn default() -> Self {
        KonnektorType::Subjunktionen
    }
}

impl fmt::Display for KonnektorType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            KonnektorType::Subjunktionen => "Nebensatz (Verb am Ende) (= Subjunktionen)",
            KonnektorType::Konjunktionen => "Hauptsatz (Position 0) (= Konjunktionen)",
            KonnektorType::Konjunktionaladverbien => {
                "Hauptsatz (Position 1) (= Konjunktionaladverbien)"
            }
            KonnektorType::Infinitivgruppe => "Infinitivgruppe",
            KonnektorType::BesonderePosition => "besondere Position",
        };
        write!(f, "{}", s)
    }
}

impl TypeTrait for KonnektorType {
    fn get_type() -> String {
        "Konnektoren".to_string()
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct CategoryWrapper {
    category: KonnektorType,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subjunktionen() {
        let json = r#"{"category": "Konnektoren mit Nebensatz (Verb am Ende) (= Subjunktionen)"}"#;
        let wrapper: CategoryWrapper = serde_json::from_str(json).unwrap();
        assert_eq!(wrapper.category, KonnektorType::Subjunktionen);
    }

    #[test]
    fn test_konjunktionen() {
        let json = r#"{"category": "Konnektoren mit Hauptsatz (Position 0) (= Konjunktionen)"}"#;
        let wrapper: CategoryWrapper = serde_json::from_str(json).unwrap();
        assert_eq!(wrapper.category, KonnektorType::Konjunktionen);
    }

    #[test]
    fn test_konjunktionaladverbien() {
        let json =
            r#"{"category": "Konnektoren mit Hauptsatz (Position 1) (= Konjunktionaladverbien)"}"#;
        let wrapper: CategoryWrapper = serde_json::from_str(json).unwrap();
        assert_eq!(wrapper.category, KonnektorType::Konjunktionaladverbien);
    }

    #[test]
    fn test_infinitivgruppe() {
        // This test already exists, shown for completeness
        let json = r#"{"category": "Konnektoren mit Infinitivgruppe"}"#;
        let wrapper: CategoryWrapper = serde_json::from_str(json).unwrap();
        assert_eq!(wrapper.category, KonnektorType::Infinitivgruppe);
    }

    #[test]
    fn test_besondere_position() {
        let json = r#"{"category": "Konnektoren mit besonderer Position"}"#;
        let wrapper: CategoryWrapper = serde_json::from_str(json).unwrap();
        assert_eq!(wrapper.category, KonnektorType::BesonderePosition);
    }
}
