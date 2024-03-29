use super::TypeTrait;
#[cfg(feature = "openapi")]
use utoipa::ToSchema;

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
pub struct AnswerRecord<T: TypeTrait>
where
    T: TypeTrait,
{
    pub detail_index: usize,
    pub was_answered: bool,
    pub correct_answer: T,
    pub user_answer: Option<T>,
}

impl<T: TypeTrait> AnswerRecord<T>
where
    T: PartialEq + Clone + Default,
{
    pub fn is_correct(&self) -> bool {
        self.was_answered && self.user_answer.clone().unwrap_or_default() == self.correct_answer
    }
}

#[cfg(test)]
mod tests {
    use serde::{Deserialize, Serialize};
    use strum::EnumIter;

    use super::*;
    #[derive(Debug, Clone, PartialEq, Deserialize, Serialize, EnumIter)]
    enum TestType {
        Type1,
        Type2,
        Unknown,
    }

    impl std::fmt::Display for TestType {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let s = match self {
                TestType::Type1 => "Type1",
                TestType::Type2 => "Type2",
                TestType::Unknown => "Unknown",
            };
            write!(f, "{}", s)
        }
    }

    impl TypeTrait for TestType {
        fn get_type() -> String {
            "TestType".to_string()
        }

        fn get_t() -> crate::model::TestType {
            crate::model::TestType::Konnektoren
        }
    }

    impl Default for TestType {
        fn default() -> Self {
            TestType::Unknown
        }
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
