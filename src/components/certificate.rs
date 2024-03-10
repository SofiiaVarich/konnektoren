use yew::prelude::*;
use crate::model::TestResult;
use web_sys::window;

#[derive(Properties, PartialEq)]
pub struct CertificateProps {
    pub test_result: TestResult,
}

#[function_component(Certificate)]
pub fn certificate(props: &CertificateProps) -> Html {
    let hostname = window().unwrap().location().host().unwrap_or_default();
    let protocol = window().unwrap().location().protocol().unwrap_or_default();

    let performance_message = format!("Performance: {:.2}%", props.test_result.performance_percentage);
    let issued_by_message = format!("Issued by {}", hostname);

    html! {
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
    }
}
