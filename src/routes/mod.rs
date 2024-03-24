use konnektoren::model::TestResult;
use serde::{Deserialize, Serialize};
use worker::*;

use crate::nft::Metadata;
use crate::services::{
    generate_and_upload_metadata, generate_metadata_response, generate_png_response,
};
use urlencoding::decode;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GenerateResult {
    pub image_cid: String,
    pub metadata_cid: String,
    pub test_result: TestResult,
    pub metadata: Metadata,
}

pub async fn handle_certificate_request(
    _req: Request,
    ctx: RouteContext<()>,
) -> worker::Result<Response> {
    if let Some(code) = ctx.param("code") {
        if code.ends_with(".png") {
            let base64_encoded = code.trim_end_matches(".png");
            let decoded_data = decode(base64_encoded).unwrap_or_default().to_string();

            let test_result = match TestResult::from_base64(&decoded_data) {
                Ok(result) => result,
                Err(_) => return Response::error(format!("Bad Request {}", &decoded_data), 400),
            };
            return generate_png_response(&test_result);
        }
    }
    Response::error("Bad Request", 400)
}

pub async fn handle_generate_request(
    _req: Request,
    ctx: RouteContext<()>,
) -> worker::Result<Response> {
    if let Some(code) = ctx.param("code") {
        let decoded_data = decode(code).unwrap_or_default().to_string();

        let test_result = match TestResult::from_base64(&decoded_data) {
            Ok(result) => result,
            Err(_) => return Response::error(format!("Bad Request {}", &decoded_data), 400),
        };

        let api_key: String = ctx.env.secret("IPFS_API_KEY").unwrap().to_string();

        return Response::from_json(
            &generate_and_upload_metadata(&test_result, api_key)
                .await
                .unwrap(),
        );
    }
    Response::error("Bad Request", 400)
}

pub async fn handle_metadata_request(
    _req: Request,
    ctx: RouteContext<()>,
) -> worker::Result<Response> {
    if let Some(code) = ctx.param("code") {
        if code.ends_with(".json") {
            let base64_encoded = code.trim_end_matches(".json");
            let decoded_data = decode(base64_encoded).unwrap_or_default().to_string();

            let test_result = match TestResult::from_base64(&decoded_data) {
                Ok(result) => result,
                Err(_) => return Response::error(format!("Bad Request {}", &decoded_data), 400),
            };
            return generate_metadata_response(&test_result);
        }
    }
    Response::error("Bad Request", 400)
}
