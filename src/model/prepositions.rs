use super::{AdjectiveDetail, Category, PrepositionType};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Prepositions {
    pub categories: Vec<Category<PrepositionType, AdjectiveDetail>>,
}

impl Default for Prepositions {
    fn default() -> Self {
        let yaml_content = include_str!("../prepositions.yml");
        serde_yaml::from_str(&yaml_content)
            .unwrap_or_else(|err| panic!("Failed to deserialize YAML content: {}", err))
    }
}

impl Prepositions {
    pub fn len(&self) -> usize {
        self.categories.iter().map(|c| c.details.len()).sum()
    }

    pub fn determine_type(&self, index: usize) -> PrepositionType {
        let mut cumulated_index = 0;

        for category in self.categories.iter() {
            if index < cumulated_index + category.details.len() {
                return category.category.clone();
            }
            cumulated_index += category.details.len();
        }

        panic!("Index out of bounds: {}", index);
    }

    pub fn get_detail_by_index(
        &self,
        index: usize,
    ) -> Option<&Category<PrepositionType, AdjectiveDetail>> {
        let mut cumulated_index = 0;

        for category in &self.categories {
            if index < cumulated_index + category.details.len() {
                let detail_index = index - cumulated_index;
                return self.categories.get(detail_index);
            }
            cumulated_index += category.details.len();
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_contains_all_enum_values_with_details() {
        let prepositions = Prepositions::default();

        let expected_types = vec![
            PrepositionType::An,
            PrepositionType::Auf,
            PrepositionType::Bei,
            PrepositionType::Fuer,
            PrepositionType::Mit,
            PrepositionType::Ueber,
            PrepositionType::Von,
            PrepositionType::Zu,
        ];

        for expected_type in expected_types {
            let category = prepositions
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
