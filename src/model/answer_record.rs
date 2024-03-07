#[derive(Debug, Clone, PartialEq)]
pub struct AnswerRecord<T>
where
    T: PartialEq + Clone + Default,
{
    pub detail_index: usize,
    pub was_answered: bool,
    pub correct_answer: T,
    pub user_answer: Option<T>,
}

impl<T> AnswerRecord<T>
where
    T: PartialEq + Clone + Default,
{
    pub fn is_correct(&self) -> bool {
        self.was_answered && self.user_answer.clone().unwrap_or_default() == self.correct_answer
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[derive(Debug, Clone, PartialEq, Default)]
    enum TestType {
        Type1,
        Type2,
        #[default]
        Unknown,
    }

    #[test]
    fn answer_record_correct() {
        let answer_record = AnswerRecord {
            detail_index: 0,
            was_answered: true,
            correct_answer: TestType::Type1,
            user_answer: Some(TestType::Type1),
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
            correct_answer: TestType::Type1,
            user_answer: Some(TestType::Type2),
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
            correct_answer: TestType::Type1,
            user_answer: None, // Or Some(TestType::Unknown) depending on how you handle unanswered
        };
        assert!(
            !answer_record.is_correct(),
            "Unanswered questions should not be marked as correct."
        );
    }
}
