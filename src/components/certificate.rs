use crate::model::TestResult;
use crate::utils::create_certificate;
use base64::engine::general_purpose;
use base64::Engine as _;
use image::ImageOutputFormat;
use std::io::Cursor;
use urlencoding::encode;
use web_sys::window;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct CertificateProps {
    pub test_result: TestResult,
}

#[function_component(Certificate)]
pub fn certificate(props: &CertificateProps) -> Html {
    let hostname = window().unwrap().location().host().unwrap_or_default();
    let protocol = window().unwrap().location().protocol().unwrap_or_default();

    let encoded_code: String = encode(&props.test_result.to_base64()).into_owned();

    let share_url = format!(
        "{}//{}/?page=results&code={}",
        protocol, hostname, encoded_code
    );

    html! {
        <div>
            <div class="certificate-image">
                <img src={create_certificate_data_url(&props.test_result, &share_url, &hostname)} />
            </div>
        </div>
    }
}

fn create_certificate_data_url(test_result: &TestResult, url: &str, issuer: &str) -> String {
    let image = create_certificate(test_result, url, issuer).unwrap();

    let mut image_data: Vec<u8> = Vec::new();
    image
        .write_to(&mut Cursor::new(&mut image_data), ImageOutputFormat::Png)
        .unwrap();
    let res_base64 = general_purpose::STANDARD.encode(image_data);
    format!("data:image/png;base64,{}", res_base64)
}
