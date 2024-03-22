use crate::model::TestResult;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct CertificateProps {
    pub test_result: TestResult,
}

#[cfg(feature = "certificate-image")]
#[function_component(Certificate)]
pub fn certificate(props: &CertificateProps) -> Html {
    use crate::utils::create_certificate_data_url;
    use urlencoding::encode;
    use web_sys::window;

    let hostname = window().unwrap().location().host().unwrap_or_default();
    let protocol = window().unwrap().location().protocol().unwrap_or_default();

    let encoded_code: String = encode(&props.test_result.to_base64()).into_owned();

    let share_url = format!(
        "{}//{}/?page=results&code={}",
        protocol, hostname, encoded_code
    );

    match create_certificate_data_url(&props.test_result, &share_url, &hostname) {
        Ok(img_src) => html! {
            <div>
                <div class="certificate-image">
                    <img src={img_src} />
                </div>
            </div>
        },
        Err(err) => html! {
            <div>
                <p>{"Error creating certificate image: "}{err}</p>
            </div>
        },
    }
}

#[cfg(not(feature = "certificate-image"))]
#[function_component(Certificate)]
pub fn certificate(_props: &CertificateProps) -> Html {
    html!()
}
