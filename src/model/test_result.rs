use crate::model::TestType;
use base64::{engine::general_purpose, Engine as _};
use ed25519_dalek::{
    ed25519::SignatureBytes, Signature, Signer, SigningKey, Verifier, VerifyingKey,
};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::str;
#[cfg(feature = "openapi")]
use utoipa::ToSchema;

fn keypair_from_static_str() -> (SigningKey, VerifyingKey) {
    let mut hasher = Sha256::new();
    hasher.update(env!("SIGNATURE_PRIVATE_KEY"));
    let result = hasher.finalize();

    let seed: [u8; 32] = result[..]
        .try_into()
        .expect("Hash output size does not match ed25519 seed size");

    let signing_key = SigningKey::from_bytes(&seed);
    let verify_key = signing_key.verifying_key();

    (signing_key, verify_key)
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
pub struct TestResult {
    pub test_type: TestType,
    pub total_questions: usize,
    pub correct_answers: usize,
    pub incorrect_answers: usize,
    pub performance_percentage: f64,
    pub player_name: String,
    pub signature: Option<Vec<u8>>,
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
        let mut buf = Vec::new();

        self.serialize(&mut rmp_serde::Serializer::new(&mut buf))
            .expect("Failed to serialize test result to msgpack.");

        general_purpose::STANDARD.encode(buf).to_string()
    }

    pub fn from_base64(encoded: &str) -> Result<Self, &'static str> {
        let decoded = match general_purpose::STANDARD.decode(encoded) {
            Ok(decoded) => decoded,
            Err(_) => return Err("Failed to decode the test result."),
        };

        match rmp_serde::from_slice(&decoded) {
            Ok(test_result) => Ok(test_result),
            Err(_) => Err("Failed to deserialize test result from msgpack."),
        }
    }

    pub fn create_signature(&mut self) {
        let (signing_key, _) = keypair_from_static_str();
        let test_result_copy = TestResult {
            test_type: self.test_type.clone(),
            total_questions: self.total_questions,
            correct_answers: self.correct_answers,
            incorrect_answers: self.incorrect_answers,
            performance_percentage: self.performance_percentage,
            player_name: self.player_name.clone(),
            signature: None,
        };

        let serialized =
            serde_cbor::to_vec(&test_result_copy).expect("Failed to serialize test result");
        let signature: Signature = signing_key.sign(&serialized);

        self.signature = Some(signature.to_bytes().to_vec());
    }

    pub fn verify(&self) -> bool {
        let (_, verifying_key) = keypair_from_static_str();

        let test_result_copy = TestResult {
            test_type: self.test_type.clone(),
            total_questions: self.total_questions,
            correct_answers: self.correct_answers,
            incorrect_answers: self.incorrect_answers,
            performance_percentage: self.performance_percentage,
            player_name: self.player_name.clone(),
            signature: None,
        };

        let serialized = serde_cbor::to_vec(&test_result_copy)
            .expect("Failed to serialize test result for verification");

        match &self.signature {
            Some(signature) => {
                let signature_bytes = SignatureBytes::try_from(signature.as_slice())
                    .expect("Failed to convert signature bytes");
                let signature = Signature::from_bytes(&signature_bytes);

                verifying_key.verify(&serialized, &signature).is_ok()
            }
            None => false,
        }
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

        assert_eq!(Ok(test_result), decoded_test_result);
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

    #[test]
    fn test_signature_verification() {
        let mut test_result = TestResult::new(TestType::Adjectives, 10, 8, 2, "Player".to_string());

        test_result.create_signature();
        let is_verified = test_result.verify();

        assert!(
            is_verified,
            "The signature should be verified successfully."
        );
    }
}
