use konnektoren::model::TestResult;
use urlencoding::decode;
use worker::*;

mod certificate;
mod nft;

mod services;
use services::{generate_json_response, generate_png_response, upload_image_to_ipfs};

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    if req.method() != Method::Get {
        return Response::error("Method Not Allowed", 405);
    }

    let url = req.url()?;
    let path = url.path();
    let (base64_encoded, response_format) = match path.strip_prefix("/certificate/") {
        Some(encoded) => {
            if encoded.ends_with(".png") {
                (encoded.trim_end_matches(".png"), "png")
            } else if encoded.ends_with(".json") {
                (encoded.trim_end_matches(".json"), "json")
            } else if encoded.ends_with(".txt") {
                (encoded.trim_end_matches(".txt"), "txt")
            } else {
                return Response::error("Unsupported format", 400);
            }
        }
        None => return Response::error("Bad Request", 400),
    };

    let decoded_data = decode(base64_encoded).unwrap_or_default().to_string();

    let test_result = match TestResult::from_base64(&decoded_data) {
        Ok(result) => result,
        Err(_) => return Response::error(format!("Bad Request {}", &decoded_data), 400),
    };

    match response_format {
        "png" => generate_png_response(&test_result),
        "json" => generate_json_response(&test_result),
        "txt" => upload_image_to_ipfs(&test_result, &env).await,
        _ => Response::error("Unsupported format", 400),
    }
}
