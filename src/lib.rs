use konnektoren::model::TestResult;
use urlencoding::decode;
use worker::*;

mod certificate;
mod nft;

mod services;
use services::{generate_json_response, generate_png_response, upload_image_to_ipfs};

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    let router = Router::new();
    router
        .get_async("/certificate/:code", |_req, ctx| async move {
            if let Some(code) = ctx.param("code") {
                if code.ends_with(".png") {
                    let base64_encoded = code.trim_end_matches(".png");
                    let decoded_data = decode(base64_encoded).unwrap_or_default().to_string();

                    let test_result = match TestResult::from_base64(&decoded_data) {
                        Ok(result) => result,
                        Err(_) => {
                            return Response::error(format!("Bad Request {}", &decoded_data), 400)
                        }
                    };
                    return generate_png_response(&test_result);
                }
            }
            Response::error("Bad Request", 400)
        })
        .get_async("/metadata/:code", |_req, ctx| async move {
            if let Some(code) = ctx.param("code") {
                if code.ends_with(".json") {
                    let base64_encoded = code.trim_end_matches(".json");
                    let decoded_data = decode(base64_encoded).unwrap_or_default().to_string();

                    let test_result = match TestResult::from_base64(&decoded_data) {
                        Ok(result) => result,
                        Err(_) => {
                            return Response::error(format!("Bad Request {}", &decoded_data), 400)
                        }
                    };
                    return generate_json_response(&test_result);
                }
            }
            Response::error("Bad Request", 400)
        })
        .get_async("/generate/:code", |_req, ctx| async move {
            if let Some(code) = ctx.param("code") {
                let decoded_data = decode(code).unwrap_or_default().to_string();

                let test_result = match TestResult::from_base64(&decoded_data) {
                    Ok(result) => result,
                    Err(_) => {
                        return Response::error(format!("Bad Request {}", &decoded_data), 400)
                    }
                };
                return upload_image_to_ipfs(&test_result, &ctx.env).await;
            }
            Response::error("Bad Request", 400)
        })
        .run(req, env)
        .await
}
