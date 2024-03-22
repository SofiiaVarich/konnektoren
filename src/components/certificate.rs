use crate::model::TestResult;
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

    #[cfg(feature = "certificate-image")]
    {
        use crate::utils::create_certificate_data_url;
        let share_url = format!(
            "{}//{}/?page=results&code={}",
            protocol, hostname, encoded_code
        );

        match create_certificate_data_url(&props.test_result, &share_url, &hostname) {
            Ok(img_src) => html! {
                    <div class="certificate-image">
                        <img src={img_src} />
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
    {
        let img_src = format!(
            "{}//{}/certificate/{}.png",
            protocol, hostname, encoded_code
        );
        html! {
            <div class="certificate-image">
                <img src={img_src} />
            </div>
        }
    }
}
