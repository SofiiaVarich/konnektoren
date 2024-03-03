use super::KonnektorType;

#[derive(Debug, Clone, PartialEq)]
pub struct AnswerRecord {
    pub detail_index: usize,
    pub was_answered: bool,
    pub correct_answer: KonnektorType,
    pub user_answer: Option<KonnektorType>,
}

impl AnswerRecord {
    pub fn is_correct(&self) -> bool {
        self.was_answered
            && self.correct_answer
                == <std::option::Option<KonnektorType> as Clone>::clone(&self.user_answer)
                    .unwrap_or_default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn answer_record_correct() {
        let answer_record = AnswerRecord {
            detail_index: 0,
            was_answered: true,
            correct_answer: KonnektorType::Subjunktionen,
            user_answer: Some(KonnektorType::Subjunktionen),
        };
        assert!(
            answer_record.is_correct(),
            "Answer should be marked as correct."
        );
    }

    #[test]
    fn answer_record_incorrect() {
        let answer_record = AnswerRecord {
            detail_index: 1,
            was_answered: true,
            correct_answer: KonnektorType::Subjunktionen,
            user_answer: Some(KonnektorType::Konjunktionen),
        };
        assert!(
            !answer_record.is_correct(),
            "Answer should be marked as incorrect."
        );
    }

    #[test]
    fn answer_record_not_answered() {
        let answer_record = AnswerRecord {
            detail_index: 2,
            was_answered: false,
            correct_answer: KonnektorType::Subjunktionen,
            user_answer: Some(KonnektorType::Subjunktionen),
        };
        assert!(
            !answer_record.is_correct(),
            "Unanswered questions should not be marked as correct."
        );
    }
}
