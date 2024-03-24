use konnektoren::model::TestResult;
use urlencoding::encode;
use worker::*;

use crate::certificate::Certificate;
use crate::nft::{Ipfs, Metadata};
use crate::routes::GenerateResult;

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

pub fn generate_metadata_response(test_result: &TestResult) -> Result<Response> {
    Response::from_json(&generate_metadata(test_result).unwrap())
}

pub fn generate_metadata(test_result: &TestResult) -> Result<Metadata> {
    let mut metadata = Metadata::from_testresult(test_result);
    metadata.image = format!(
        "https://konnektoren.help/certificate/{}.png",
        encode(&test_result.to_base64()).into_owned()
    );
    Ok(metadata)
}

pub async fn upload_image_to_ipfs(test_result: &TestResult, api_key: String) -> Result<String> {
    let issuer = "konnektoren.help";
    let encoded_code: String = encode(&test_result.to_base64()).into_owned();
    let url = format!(
        "https://konnektoren.help/?page=results&code={}",
        encoded_code
    );
    let certificate = Certificate::new(issuer.to_string(), test_result.clone(), url);

    let bytes = certificate.to_png().unwrap();
    let ipfs = Ipfs::new(api_key);
    Ok(ipfs
        .upload(bytes, "certificate.png".to_string())
        .await
        .unwrap())
}

pub async fn generate_and_upload_metadata(
    test_result: &TestResult,
    api_key: String,
) -> Result<GenerateResult> {
    let image_cid = upload_image_to_ipfs(test_result, api_key.clone())
        .await
        .unwrap();

    let metadata = Metadata::from_testresult_and_image_cid(test_result, image_cid.clone());

    let metadata_cid = Ipfs::new(api_key)
        .upload(
            metadata.to_json().as_bytes().to_vec(),
            "metadata.json".to_string(),
        )
        .await
        .unwrap();

    Ok(GenerateResult {
        image_cid,
        metadata_cid,
        test_result: test_result.clone(),
        metadata,
    })
}
