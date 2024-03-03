use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum KonnektorType {
    #[serde(rename = "Konnektoren mit Infinitivgruppe")]
    Infinitivgruppe,
    #[serde(rename = "Konnektoren mit Nebensatz")]
    Nebensatz,
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
    fn test_konnektor_type() {
        // Adjust the test to deserialize into the wrapper struct
        let json = r#"{"category": "Konnektoren mit Infinitivgruppe"}"#;
        let wrapper: CategoryWrapper = serde_json::from_str(json).unwrap();
        assert_eq!(wrapper.category, KonnektorType::Infinitivgruppe);
    }
}
