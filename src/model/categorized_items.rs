use super::adjective_type::AdjectiveType;
use super::category::Category;
use super::konnektor_detail::KonnektorDetail;
use super::konnektor_type::KonnektorType;
use super::{adjective_detail::AdjectiveDetail, TypeTrait};
use super::{DetailTrait, VerbDetail, VerbType};
use serde::{Deserialize, Serialize};
#[cfg(feature = "openapi")]
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
pub struct CategorizedItems<T: TypeTrait, D: DetailTrait> {
    pub categories: Vec<Category<T, D>>,
}

impl<T: TypeTrait, D: DetailTrait> CategorizedItems<T, D> {
    pub fn len(&self) -> usize {
        self.categories.iter().map(|c| c.details.len()).sum()
    }

    pub fn determine_type(&self, index: usize) -> T {
        let mut cumulated_index = 0;

        for category in self.categories.iter() {
            if index < cumulated_index + category.details.len() {
                return category.category.clone();
            }
            cumulated_index += category.details.len();
        }

        panic!("Index out of bounds: {}", index);
    }

    pub fn get_detail_by_index(&self, index: usize) -> Option<&D> {
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

impl Default for CategorizedItems<AdjectiveType, AdjectiveDetail> {
    fn default() -> Self {
        let yaml_content = include_str!("../adjectives.yml");

        let prepositions: CategorizedItems<AdjectiveType, AdjectiveDetail> =
            serde_yaml::from_str(&yaml_content)
                .unwrap_or_else(|err| panic!("Failed to deserialize YAML content: {}", err));

        prepositions
    }
}

impl Default for CategorizedItems<KonnektorType, KonnektorDetail> {
    fn default() -> Self {
        let yaml_content = include_str!("../konnektoren.yml");

        let konnektoren: CategorizedItems<KonnektorType, KonnektorDetail> =
            serde_yaml::from_str(&yaml_content)
                .unwrap_or_else(|err| panic!("Failed to deserialize YAML content: {}", err));

        konnektoren
    }
}

impl Default for CategorizedItems<VerbType, VerbDetail> {
    fn default() -> Self {
        let yaml_content = include_str!("../verbs.yml");

        let verbs: CategorizedItems<VerbType, VerbDetail> = serde_yaml::from_str(&yaml_content)
            .unwrap_or_else(|err| panic!("Failed to deserialize YAML content: {}", err));

        verbs
    }
}

#[cfg(test)]
mod tests {

    use super::super::adjective_detail::AdjectiveDetail;
    use super::super::adjective_type::AdjectiveType;
    use super::super::category::Category;
    use super::super::konnektor_detail::KonnektorDetail;
    use super::super::konnektor_type::KonnektorType;
    use super::*;

    #[test]
    fn default_contains_all_enum_values_with_details() {
        let prepositions = CategorizedItems::<AdjectiveType, AdjectiveDetail>::default();
        let konnektoren = CategorizedItems::<KonnektorType, KonnektorDetail>::default();

        let expected_prepositions = vec![
            AdjectiveType::An,
            AdjectiveType::Auf,
            AdjectiveType::Bei,
            AdjectiveType::Fuer,
            AdjectiveType::Mit,
            AdjectiveType::Ueber,
            AdjectiveType::Von,
            AdjectiveType::Zu,
        ];
        let expected_konnektoren = vec![
            KonnektorType::Konjunktionen,
            KonnektorType::Subjunktionen,
            KonnektorType::Konjunktionaladverbien,
            KonnektorType::Infinitivgruppe,
            KonnektorType::BesonderePosition,
        ];

        assert_eq!(prepositions.categories.len(), expected_prepositions.len());
        assert_eq!(konnektoren.categories.len(), expected_konnektoren.len());

        for expected_type in expected_konnektoren {
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
        let konnektoren = CategorizedItems {
            categories: vec![
                Category {
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
                Category {
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
        let konnektoren = CategorizedItems {
            categories: vec![
                Category {
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
                Category {
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
        let konnektoren = CategorizedItems::<KonnektorType, KonnektorDetail>::default();

        let detail = konnektoren.get_detail_by_index(999); // Use an index that's clearly out of bounds

        assert!(
            detail.is_none(),
            "Attempting to get a detail by an out-of-bounds index should return None."
        );
    }

    #[test]
    fn test_determine_type() {
        let konnektoren = CategorizedItems::<KonnektorType, KonnektorDetail>::default();

        let expected_type = KonnektorType::Infinitivgruppe;

        let determined_type = konnektoren.determine_type(0);
        assert_eq!(
            determined_type, expected_type,
            "The determined type for index {} should be {:?}",
            0, expected_type
        );
    }
}
