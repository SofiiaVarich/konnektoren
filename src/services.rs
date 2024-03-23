use konnektoren::model::TestResult;
use urlencoding::encode;
use worker::*;

use crate::certificate::Certificate;
use crate::nft::{Ipfs, Metadata};

pub fn generate_png_response(test_result: &TestResult) -> Result<Response> {
    let issuer = "konnektoren.help";
    let encoded_code: String = encode(&test_result.to_base64()).into_owned();
    let url = format!(
        "https://konnektoren.help/?page=results&code={}",
        encoded_code
    );

    let certificate = Certificate::new(issuer.to_string(), test_result.clone(), url);
    match certificate.to_png() {
        Ok(bytes) => Response::from_bytes(bytes).map(|mut res| {
            let _ = res.headers_mut().set("Content-Type", "image/png");
            res
        }),
        Err(_) => Response::error("Internal Server Error", 500),
    }
}

pub fn generate_json_response(test_result: &TestResult) -> Result<Response> {
    let mut metadata = Metadata::from_testresults(test_result);
    metadata.image = format!(
        "https://konnektoren.help/certificate/{}.png",
        encode(&test_result.to_base64()).into_owned()
    );
    Response::from_json(&metadata)
}

pub async fn upload_image_to_ipfs(test_result: &TestResult, env: &Env) -> Result<Response> {
    let certificate = Certificate::new(
        "konnektoren.help".to_string(),
        test_result.clone(),
        "".to_string(),
    );
    let api_key: String = env.secret("IPFS_API_KEY")?.to_string();
    match certificate.to_png() {
        Ok(bytes) => {
            let ipfs = Ipfs::new(bytes, api_key);
            match ipfs.upload().await {
                Ok(hash) => Response::from_json(&hash),
                Err(_) => Response::error("Internal Server Error", 500),
            }
        }
        Err(_) => Response::error("Internal Server Error", 500),
    }
}
