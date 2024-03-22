use image::ImageOutputFormat;
use konnektoren::model::TestResult;
use konnektoren::utils::create_certificate;
use std::io::Cursor;
use urlencoding::decode;
use worker::*;

#[event(fetch)]
pub async fn main(req: Request, _env: Env, _ctx: Context) -> Result<Response> {
    // Only proceed for GET requests
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
        Err(_) => return Response::error(format!("Bad Request {}", base64_encoded), 400),
    };

    let issuer = "Your Organization"; // Customize this
    let url = "https://example.com"; // Customize this
    match create_certificate(&test_result, url, issuer) {
        Ok(image) => {
            // Convert the DynamicImage to PNG
            let mut bytes: Vec<u8> = Vec::new();
            match image.write_to(&mut Cursor::new(&mut bytes), ImageOutputFormat::Png) {
                Ok(_) => {
                    // Create a response with the PNG data
                    Response::from_bytes(bytes).map(|mut res| {
                        let _ = res.headers_mut().set("Content-Type", "image/png");
                        res
                    })
                }
                Err(_) => Response::error("Internal Server Error", 500),
            }
        }
        Err(_) => Response::error("Internal Server Error", 500),
    }
}
