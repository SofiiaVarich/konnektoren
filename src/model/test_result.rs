use crate::model::TestType;
use base64::{engine::general_purpose, Engine as _};
use serde::{Deserialize, Serialize};
use std::str;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct TestResult {
    test_type: TestType,
    total_questions: usize,
    correct_answers: usize,
    incorrect_answers: usize,
    performance_percentage: f64,
    player_name: String,
    signature: Option<String>,
}

impl TestResult {
    pub fn new(
        test_type: TestType,
        total_questions: usize,
        correct_answers: usize,
        incorrect_answers: usize,
        player_name: String,
    ) -> Self {
        let performance_percentage = (correct_answers as f64 / total_questions as f64) * 100.0;
        TestResult {
            test_type,
            total_questions,
            correct_answers,
            incorrect_answers,
            performance_percentage,
            player_name,
            signature: None,
        }
    }

    pub fn to_base64(&self) -> String {
        let serialized = serde_json::to_string(&self).expect("Failed to serialize test result");

        general_purpose::STANDARD.encode(&serialized).to_string()
    }

    pub fn from_base64(encoded: &str) -> Self {
        let decoded = general_purpose::STANDARD
            .decode(encoded)
            .expect("Failed to decode test result");
        let decoded_str =
            str::from_utf8(&decoded).expect("Failed to convert decoded bytes to string");

        serde_json::from_str(decoded_str).expect("Failed to deserialize test result")
    }

    pub fn create_signature(&mut self) {
        self.signature = Some("signature".to_string());
    }

    pub fn has_correct_signature(&self) -> bool {
        self.signature == Some("signature".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base64_serialization_deserialization() {
        let test_result = TestResult::new(TestType::Adjectives, 10, 8, 2, "Player".to_string());
        let base64_encoded = test_result.to_base64();
        let decoded_test_result = TestResult::from_base64(&base64_encoded);

        assert_eq!(test_result, decoded_test_result);
    }

    #[test]
    fn test_result_new() {
        let test_result = TestResult::new(TestType::Adjectives, 10, 8, 2, "Player".to_string());
        assert_eq!(test_result.test_type, TestType::Adjectives);
        assert_eq!(test_result.total_questions, 10);
        assert_eq!(test_result.correct_answers, 8);
        assert_eq!(test_result.incorrect_answers, 2);
        assert_eq!(test_result.performance_percentage, 80.0);
    }
}
