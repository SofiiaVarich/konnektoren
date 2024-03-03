use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum KonnektorType {
    #[serde(rename = "Konnektoren mit Nebensatz (= Subjunktionen)")]
    Subjunktionen,
    #[serde(rename = "Konnektoren mit Hauptsatz (Position 0) (= Konjunktionen)")]
    Konjunktionen,
    #[serde(rename = "Konnektoren mit Hauptsatz (Position 1) (= Konjunktionaladverbien)")]
    Konjunktionaladverbien,
    #[serde(rename = "Konnektoren mit Infinitivgruppe")]
    Infinitivgruppe,
    #[serde(rename = "Konnektoren mit besonderer Position")]
    BesonderePosition,
    Other(String),
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
        let json = r#"{"category": "Konnektoren mit Nebensatz (= Subjunktionen)"}"#;
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
