use super::konnektor_category::KonnektorCategory;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Konnektoren {
    pub categories: Vec<KonnektorCategory>,
}

impl Default for Konnektoren {
    fn default() -> Self {
        let yaml_content = include_str!("../konnektoren.yml");

        let konnektoren: Konnektoren = serde_yaml::from_str(&yaml_content)
            .unwrap_or_else(|err| panic!("Failed to deserialize YAML content: {}", err));

        konnektoren
    }
}

#[cfg(test)]
mod tests {
    use super::super::konnektor_type::KonnektorType;
    use super::*;

    #[test]
    fn default_contains_all_enum_values_with_details() {
        let konnektoren = Konnektoren::default();

        let expected_types = vec![
            KonnektorType::Konjunktionen,
            KonnektorType::Subjunktionen,
            KonnektorType::Konjunktionaladverbien,
            KonnektorType::Infinitivgruppe,
            KonnektorType::BesonderePosition,
        ];

        for expected_type in expected_types {
            let category = konnektoren
                .categories
                .iter()
                .find(|c| c.category == expected_type);
            assert!(
                category.is_some(),
                "Category {:?} is missing",
                expected_type
            );
            assert!(
                category.unwrap().details.len() > 0,
                "Details for {:?} are empty",
                expected_type
            );
        }
    }
}
