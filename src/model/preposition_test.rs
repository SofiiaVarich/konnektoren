use super::AdjectiveDetail;
use super::AnswerRecord;
use super::PrepositionType;
use super::Prepositions;
use rand::seq::SliceRandom;

#[derive(Debug, Clone, PartialEq)]
pub struct PrepositionTest {
    pub prepositions: Prepositions,
    pub random_indices: Vec<usize>,
    pub current_index: usize,
    pub answers: Vec<AnswerRecord<PrepositionType>>,
}

impl PrepositionTest {
    pub fn new(prepositions: &Prepositions) -> Self {
        let details: Vec<AdjectiveDetail> = prepositions
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
                let correct_preposition = prepositions.determine_type(index); // You need to implement this method based on your domain
                AnswerRecord {
                    detail_index: index,
                    was_answered: false,
                    correct_answer: correct_preposition,
                    user_answer: None,
                }
            })
            .collect();

        Self {
            prepositions: prepositions.clone(),
            random_indices: indices,
            current_index: 0,
            answers,
        }
    }
}
