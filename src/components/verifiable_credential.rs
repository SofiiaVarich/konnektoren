use crate::model::TestResult;
use crate::utils::create_test_result_vc;
use qrcode::render::svg;
use qrcode::{EcLevel, QrCode};
use web_sys::window;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct VerifiableCredentialProps {
    pub test_result: TestResult,
}

#[function_component(VerifiableCredential)]
pub fn verifiable_credential(props: &VerifiableCredentialProps) -> Html {
    let hostname = window().unwrap().location().host().unwrap_or_default();

    let vc = create_test_result_vc(&props.test_result, &hostname);
    let vc = serde_json::to_string(&vc).expect("Failed to serialize Verifiable Credential");

    let qr_code =
        QrCode::with_error_correction_level(&vc, EcLevel::H).expect("Failed to create QR code");

    let svg = qr_code
        .render()
        .min_dimensions(250, 250)
        .dark_color(svg::Color("black"))
        .light_color(svg::Color("white"))
        .build();
    let svg = Html::from_html_unchecked(AttrValue::from(svg));

    html! {
        <div>
            <h2>{"Verifiable Credential"}</h2>
            <div class="verifiable-credential">
                {svg}
            </div>
        </div>
    }
}
