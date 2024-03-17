use crate::model::TestResult;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct VerifiableCredential<T> {
    #[serde(rename = "@context")]
    context: Vec<String>,
    id: Option<String>,
    #[serde(rename = "type")]
    credential_type: Vec<String>,
    credential_subject: T,
    issuer: String,
    issuance_date: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TestResultCredential {
    name: String,
    performance: f64,
}

pub fn create_test_result_vc(
    test_result: &TestResult,
    issuer: &String,
) -> VerifiableCredential<TestResultCredential> {
    VerifiableCredential {
        context: vec![
            "https://www.w3.org/2018/credentials/v1".into(),
            "https://example.com/vocab.json".into(),
        ],
        id: Some(
            format!(
                "https://{}/credentials/{}",
                issuer,
                Uuid::new_v4().to_string()
            )
            .into(),
        ),
        credential_type: vec!["VerifiableCredential".into(), "TestResultCredential".into()],
        credential_subject: TestResultCredential {
            name: test_result.player_name.clone(),
            performance: test_result.performance_percentage,
        },
        issuer: format!("did:web:{}", issuer).into(),
        issuance_date: Utc::now().to_rfc3339(),
    }
}
