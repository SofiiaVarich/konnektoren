use crate::nft::Metadata;
use crate::services::{
    generate_and_upload_metadata, generate_metadata_response, generate_png_response,
    load_leaderboard, mint_nft, update_leaderboard,
};
use konnektoren::model::leaderboard::Leaderboard;
use konnektoren::model::TestResult;
use serde::{Deserialize, Serialize};
use urlencoding::decode;
use worker::*;

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
            update_leaderboard(&ctx.kv("KONNEKTOREN_LEADERBOARD")?, &test_result).await?;
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

pub async fn handle_leaderboard_request(
    _req: Request,
    ctx: RouteContext<()>,
) -> worker::Result<Response> {
    let kv = ctx.kv("KONNEKTOREN_LEADERBOARD")?;
    let leaderboard: Leaderboard = load_leaderboard(&kv).await?;
    Response::from_json(&leaderboard)
}

pub async fn handle_mint_request(req: Request, ctx: RouteContext<()>) -> worker::Result<Response> {
    let parsed_url = Url::parse(req.url()?.as_str())?;

    let mut query_pairs = parsed_url.query_pairs();
    let receiver: Option<String> = query_pairs
        .clone()
        .find(|(key, _)| key == "receiver")
        .map(|(_, value)| value.into());
    let code: Option<String> = query_pairs
        .find(|(key, _)| key == "code")
        .map(|(_, value)| value.into());

    log::info!("Receiver: {:?}, Code: {:?}", receiver, code);

    if let (Some(receiver), Some(code)) = (receiver, code) {
        let api_key: String = ctx.secret("UNDERDOG_API_KEY")?.to_string();
        let project_id: String = ctx.var("UNDERDOG_PROJECT_ID")?.to_string();

        let decoded_data = match decode(&code) {
            Ok(data) => data,
            Err(_) => return Response::error("Bad Request", 400),
        };

        let test_result = match TestResult::from_base64(&decoded_data) {
            Ok(result) => result,
            Err(_) => return Response::error(format!("Bad Request {}", &decoded_data), 400),
        };

        log::info!("Test Result: {:?}", test_result);

        let mint_address = match mint_nft(&test_result, receiver.clone(), project_id, api_key).await
        {
            Ok(address) => address,
            Err(err) => return Response::error(format!("Error minting: {err}"), 500),
        };

        return Response::from_json(&serde_json::json!({
            "receiver": receiver,
            "address": mint_address.to_string(),
            "page": format!("https://xray.helius.xyz/token/{}?network=devnet", mint_address)
        }));
    }

    Response::error("Bad Request", 400)
}
