use super::KonnektorType;

#[derive(Debug, Clone, PartialEq)]
pub struct AnswerRecord {
    pub detail_index: usize,
    pub was_answered: bool,
    pub correct_answer: KonnektorType,
    pub user_answer: Option<KonnektorType>,
}
