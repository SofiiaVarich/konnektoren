use konnektoren::model::TestResult;
use urlencoding::{decode, encode};
use worker::*;

mod certificate;
mod nft;

use certificate::Certificate;
use nft::Metadata;

#[event(fetch)]
pub async fn main(req: Request, _env: Env, _ctx: Context) -> Result<Response> {
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
        _ => Response::error("Unsupported format", 400),
    }
}

fn generate_png_response(test_result: &TestResult) -> Result<Response> {
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

fn generate_json_response(test_result: &TestResult) -> Result<Response> {
    let mut metadata = Metadata::from_testresults(test_result);
    metadata.image = format!(
        "https://konnektoren.help/certificate/{}.png",
        encode(&test_result.to_base64()).into_owned()
    );
    let json = serde_json::to_string(&metadata).unwrap();
    Response::ok(json).map(|mut res| {
        let _ = res.headers_mut().set("Content-Type", "application/json");
        res
    })
}
