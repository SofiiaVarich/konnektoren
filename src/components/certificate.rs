use crate::model::TestResult;
use qrcode::render::svg;
use qrcode::{EcLevel, QrCode};
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

    let performance_message = format!(
        "Performance: {:.2}%",
        props.test_result.performance_percentage
    );
    let issued_by_message = format!("Issued by {}", hostname);

    let encoded_code: String = props.test_result.to_base64();
    let share_url = format!(
        "{}//{}/?page=results&code={}",
        protocol, hostname, encoded_code
    );
    let qr_code = Html::from_html_unchecked(AttrValue::from(generate_qr_code_data_url(&share_url)));

    html! {
        <div>
            <div class="certificate">
                <h2>{ "Certificate of Completion" }</h2>
                <p>
                    { format!("{} has completed the {} test with {} correct answers out of {} total questions.",
                        props.test_result.player_name,
                        props.test_result.test_type,
                        props.test_result.correct_answers,
                        props.test_result.total_questions) }
                </p>
                <p>{ performance_message }</p>
                <p><a href={format!("{}//{}", protocol, hostname)} target="_blank">{ issued_by_message }</a></p>
            </div>
            <div class="qr-code">
                { qr_code }
            </div>
        </div>
    }
}

fn generate_qr_code_data_url(url: &str) -> String {
    let code = QrCode::with_error_correction_level(url, EcLevel::H).unwrap();
    let image = code
        .render()
        .min_dimensions(250, 250)
        .dark_color(svg::Color("#800000"))
        .light_color(svg::Color("#ffffff"))
        .build();
    image
}
