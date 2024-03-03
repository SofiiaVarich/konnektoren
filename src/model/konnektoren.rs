use super::konnektor_category::KonnektorCategory;
use super::konnektor_detail::KonnektorDetail;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
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

impl Konnektoren {
    pub fn len(&self) -> usize {
        self.categories.iter().map(|c| c.details.len()).sum()
    }

    pub fn get_detail_by_index(&self, index: usize) -> Option<&KonnektorDetail> {
        let mut cumulated_index = 0;

        for category in &self.categories {
            if index < cumulated_index + category.details.len() {
                let detail_index = index - cumulated_index;
                return category.details.get(detail_index);
            }
            cumulated_index += category.details.len();
        }

        None
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

    #[test]
    fn test_konnektoren_length() {
        let konnektoren = Konnektoren {
            categories: vec![
                KonnektorCategory {
                    category: KonnektorType::Konjunktionen,
                    details: vec![
                        KonnektorDetail {
                            konnektor: "Konnektor 1".to_string(),
                            example: "Example 1".to_string(),
                        },
                        KonnektorDetail {
                            konnektor: "Konnektor 2".to_string(),
                            example: "Example 2".to_string(),
                        },
                    ],
                },
                KonnektorCategory {
                    category: KonnektorType::Subjunktionen,
                    details: vec![KonnektorDetail {
                        konnektor: "Konnektor 3".to_string(),
                        example: "Example 3".to_string(),
                    }],
                },
            ],
        };

        let length = konnektoren.len();

        assert_eq!(
            length, 3,
            "The length should correctly reflect the total number of KonnektorDetail items."
        );
    }

    #[test]
    fn test_get_detail_by_index() {
        let konnektoren = Konnektoren {
            categories: vec![
                KonnektorCategory {
                    category: KonnektorType::Konjunktionen,
                    details: vec![
                        KonnektorDetail {
                            konnektor: "Konnektor 1".to_string(),
                            example: "Example 1".to_string(),
                        },
                        KonnektorDetail {
                            konnektor: "Konnektor 2".to_string(),
                            example: "Example 2".to_string(),
                        },
                    ],
                },
                KonnektorCategory {
                    category: KonnektorType::Subjunktionen,
                    details: vec![KonnektorDetail {
                        konnektor: "Konnektor 3".to_string(),
                        example: "Example 3".to_string(),
                    }],
                },
            ],
        };

        let detail = konnektoren.get_detail_by_index(1).unwrap();

        assert_eq!(
            detail.konnektor, "Konnektor 2",
            "The konnektor at index 1 should be 'Konnektor 2'."
        );
        assert_eq!(
            detail.example, "Example 2",
            "The example for 'Konnektor 2' should be 'Example 2'."
        );
    }

    #[test]
    fn test_get_detail_by_index_out_of_bounds() {
        let konnektoren = Konnektoren::default();

        let detail = konnektoren.get_detail_by_index(999); // Use an index that's clearly out of bounds

        assert!(
            detail.is_none(),
            "Attempting to get a detail by an out-of-bounds index should return None."
        );
    }
}
