use super::AdjectiveDetail;
use super::AdjectiveType;
use super::AnswerRecord;
use super::CategorizedItems;
use super::DetailTrait;
use super::KonnektorDetail;
use super::KonnektorType;
use super::TypeTrait;
use rand::seq::SliceRandom;

#[derive(Debug, Clone, PartialEq)]
pub struct CategorizedTest<T: TypeTrait, D: DetailTrait> {
    pub items: CategorizedItems<T, D>,
    pub random_indices: Vec<usize>,
    pub current_index: usize,
    pub answers: Vec<AnswerRecord<T>>,
}

impl Default for CategorizedTest<KonnektorType, KonnektorDetail> {
    fn default() -> Self {
        Self::new(&CategorizedItems::default())
    }
}

impl Default for CategorizedTest<AdjectiveType, AdjectiveDetail> {
    fn default() -> Self {
        Self::new(&CategorizedItems::default())
    }
}

impl<T: TypeTrait, D: DetailTrait> CategorizedTest<T, D> {
    pub fn new(items: &CategorizedItems<T, D>) -> Self {
        let details: Vec<D> = items
            .categories
            .iter()
            .flat_map(|category| category.details.clone())
            .collect();

        let mut indices: Vec<usize> = (0..details.len()).collect();
        let mut rng = rand::thread_rng();
        indices.shuffle(&mut rng);

        let answers = indices
            .iter()
            .map(|&index| {
                let correct_answer = items.determine_type(index);
                AnswerRecord {
                    detail_index: index,
                    was_answered: false,
                    correct_answer,
                    user_answer: None,
                }
            })
            .collect();

        Self {
            items: items.clone(),
            random_indices: indices,
            current_index: 0,
            answers,
        }
    }

    pub fn next(&mut self) {
        if self.current_index < self.random_indices.len() - 1 {
            self.current_index += 1;
        }
    }

    pub fn prev(&mut self) {
        if self.current_index > 0 {
            self.current_index -= 1;
        }
    }

    pub fn len(&self) -> usize {
        self.random_indices.len()
    }

    pub fn current(&self) -> Option<&D> {
        let index = self.random_indices.get(self.current_index)?;
        self.items.get_detail_by_index(*index)
    }

    pub fn current_index(&self) -> usize {
        self.current_index
    }

    pub fn answer_current(&mut self, user_answer: T) {
        if let Some(record) = self.answers.get_mut(self.current_index) {
            record.was_answered = true;
            record.user_answer = Some(user_answer);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::Category;
    use crate::model::{KonnektorDetail, KonnektorType};

    fn mock_konnektoren() -> CategorizedItems<KonnektorType, KonnektorDetail> {
        CategorizedItems {
            categories: vec![
                Category {
                    category: KonnektorType::Konjunktionen,
                    details: vec![KonnektorDetail {
                        konnektor: "Konnektor 1".to_string(),
                        example: "Example 1".to_string(),
                    }],
                },
                Category {
                    category: KonnektorType::Subjunktionen,
                    details: vec![KonnektorDetail {
                        konnektor: "Konnektor 2".to_string(),
                        example: "Example 2".to_string(),
                    }],
                },
            ],
        }
    }

    #[test]
    fn test_default() {
        let test = CategorizedTest::<KonnektorType, KonnektorDetail>::default();
        assert!(test.len() > 0, "The length should be greater than 0.");
    }

    #[test]
    fn test_konnektor_test_initialization() {
        let konnektoren = mock_konnektoren();
        let test = CategorizedTest::new(&konnektoren);

        assert!(
            !test.random_indices.is_empty(),
            "Random indices should not be empty after initialization."
        );
        assert_eq!(
            test.current_index, 0,
            "Current index should be 0 after initialization."
        );
    }

    #[test]
    fn test_konnektor_test_navigation() {
        let konnektoren = mock_konnektoren();
        let mut test = CategorizedTest::new(&konnektoren);

        test.next();
        assert!(
            test.current_index <= test.random_indices.len() - 1,
            "Current index should not exceed the number of details."
        );

        test.prev();
        assert_eq!(
            test.current_index, 0,
            "Current index should return to 0 after calling prev."
        );
    }

    #[test]
    fn test_konnektor_test_current_detail() {
        let konnektoren = mock_konnektoren();
        let test = CategorizedTest::new(&konnektoren);

        if let Some(detail) = test.current() {
            assert!(
                detail.konnektor.contains("Konnektor"),
                "The current detail should contain 'Konnektor'."
            );
            assert!(
                detail.example.contains("Example"),
                "The example should contain 'Example'."
            );
        } else {
            panic!("Current detail should not be None.");
        }
    }
}
