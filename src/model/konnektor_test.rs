use super::AnswerRecord;
use super::KonnektorDetail;
use super::KonnektorType;
use super::Konnektoren;
use rand::seq::SliceRandom;

pub struct KonnektorTest {
    konnektoren: Konnektoren,
    random_indices: Vec<usize>,
    current_index: usize,
    answers: Vec<AnswerRecord>,
}

impl KonnektorTest {
    pub fn new(konnektoren: &Konnektoren) -> Self {
        let details: Vec<KonnektorDetail> = konnektoren
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
                let correct_answer = konnektoren.determine_type(index);
                AnswerRecord {
                    detail_index: index,
                    was_answered: false,
                    correct_answer,
                    user_answer: None,
                }
            })
            .collect();

        Self {
            konnektoren: konnektoren.clone(),
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

    pub fn current(&self) -> Option<&KonnektorDetail> {
        let index = self.random_indices.get(self.current_index)?;
        self.konnektoren.get_detail_by_index(*index)
    }

    pub fn answer_current(&mut self, user_answer: KonnektorType) {
        if let Some(record) = self.answers.get_mut(self.current_index) {
            record.was_answered = true;
            record.user_answer = Some(user_answer);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::KonnektorCategory;
    use crate::model::KonnektorType;

    // Utility function to create a mock Konnektoren instance for testing
    fn mock_konnektoren() -> Konnektoren {
        Konnektoren {
            categories: vec![
                KonnektorCategory {
                    category: KonnektorType::Konjunktionen,
                    details: vec![KonnektorDetail {
                        konnektor: "Konnektor 1".to_string(),
                        example: "Example 1".to_string(),
                    }],
                },
                KonnektorCategory {
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
    fn test_konnektor_test_initialization() {
        let konnektoren = mock_konnektoren();
        let test = KonnektorTest::new(&konnektoren);

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
        let mut test = KonnektorTest::new(&konnektoren);

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
        let test = KonnektorTest::new(&konnektoren);

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
