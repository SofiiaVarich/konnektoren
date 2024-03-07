use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
pub enum PrepositionType {
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

impl Default for PrepositionType {
    fn default() -> Self {
        PrepositionType::An
    }
}

impl fmt::Display for PrepositionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            PrepositionType::An => "an",
            PrepositionType::Auf => "auf",
            PrepositionType::Bei => "bei",
            PrepositionType::Fuer => "für",
            PrepositionType::Mit => "mit",
            PrepositionType::Ueber => "über",
            PrepositionType::Von => "von",
            PrepositionType::Zu => "zu",
        };
        write!(f, "{}", s)
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct CategoryWrapper {
    category: PrepositionType,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_an() {
        let json = r#"{"category": "an"}"#;
        let wrapper: CategoryWrapper = serde_json::from_str(json).unwrap();
        assert_eq!(wrapper.category, PrepositionType::An);
    }
}
