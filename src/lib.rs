use image::ImageOutputFormat;
use konnektoren::model::TestResult;
use konnektoren::utils::create_certificate;
use std::io::Cursor;
use urlencoding::{decode, encode};
use worker::*;

mod certificate;
use certificate::Certificate;

#[event(fetch)]
pub async fn main(req: Request, _env: Env, _ctx: Context) -> Result<Response> {
    if req.method() != Method::Get {
        return Response::error("Method Not Allowed", 405);
    }

    let url = req.url()?;
    let path = url.path();
    let base64_encoded = match path.strip_prefix("/certificate/") {
        Some(encoded) => {
            let data = encoded.trim_end_matches(".png");
            decode(data).unwrap_or_default().to_string()
        }
        None => return Response::error("Bad Request", 400),
    };

    let test_result = match TestResult::from_base64(&base64_encoded) {
        Ok(result) => result,
        Err(_) => return Response::error(format!("Bad Request {}", &base64_encoded), 400),
    };

    let issuer = "konnektoren.help";

    let encoded_code: String = encode(&test_result.to_base64()).into_owned();

    let url = format!(
        "https://konnektoren.help/?page=results&code={}",
        encoded_code
    );

    match Certificate::from_base64(&base64_encoded) {
        Ok(mut certificate) => {
            certificate.issuer = issuer.to_string();
            certificate.url = url;
            let bytes = certificate.to_png().unwrap();
            Response::from_bytes(bytes).map(|mut res| {
                let _ = res.headers_mut().set("Content-Type", "image/png");
                res
            })
        }
        Err(_) => Response::error("Internal Server Error", 500),
    }
}
