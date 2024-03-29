use konnektoren::model::TestResult;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Attribute {
    trait_type: String,
    value: serde_json::Value,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Metadata {
    pub name: String,
    pub description: String,
    pub image: String,
    pub attributes: Vec<Attribute>,
}

impl Metadata {
    pub fn from_testresult(test_result: &TestResult) -> Self {
        let attributes = vec![
            Attribute {
                trait_type: "Test Type".to_string(),
                value: serde_json::Value::String(test_result.test_type.to_string()),
            },
            Attribute {
                trait_type: "Total Questions".to_string(),
                value: serde_json::Value::Number(test_result.total_questions.into()),
            },
            Attribute {
                trait_type: "Correct Answers".to_string(),
                value: serde_json::Value::Number(test_result.correct_answers.into()),
            },
            Attribute {
                trait_type: "Incorrect Answers".to_string(),
                value: serde_json::Value::Number(test_result.incorrect_answers.into()),
            },
            Attribute {
                trait_type: "Performance Percentage".to_string(),
                value: serde_json::Value::Number(
                    serde_json::Number::from_f64(test_result.performance_percentage.into())
                        .expect("Performance percentage conversion error"),
                ),
            },
        ];

        Metadata {
            name: format!("Certificate of Achievement for {}", test_result.player_name),
            description: "This NFT certifies that the named individual has successfully completed the test with specified performance metrics.".to_string(),
            image: "URL_TO_CERTIFICATE_IMAGE".to_string(),
            attributes,
        }
    }

    pub fn from_testresult_and_image_cid(test_result: &TestResult, image_cid: String) -> Self {
        let mut metadata = Metadata::from_testresult(test_result);
        metadata.image = format!("https://ipfs.io/ipfs/{}/certificate.png", image_cid);
        metadata
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(self).expect("Metadata serialization error")
    }
}
