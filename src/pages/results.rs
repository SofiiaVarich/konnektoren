use yew::prelude::*;

use crate::model::TestResult;

#[derive(Properties, PartialEq)]
pub struct ResultsProps {
    pub code: Option<String>,
}

#[function_component(Results)]
pub fn results(props: &ResultsProps) -> Html {
    let test_result = TestResult::from_base64(props.code.as_ref().unwrap_or(&"".to_string()));

    match test_result {
        Ok(test_result) => html! {
            <div class="results-page">
                <h1>{ "Test Results" }</h1>
                <p>{ format!("Test Type: {}", test_result.test_type) }</p>
                <p>{ format!("Total Questions: {}", test_result.total_questions) }</p>
                <p>{ format!("Correct Answers: {}", test_result.correct_answers) }</p>
                <p>{ format!("Incorrect Answers: {}", test_result.incorrect_answers) }</p>
                <p>{ format!("Performance: {:.2}%", test_result.performance_percentage) }</p>
                <p>{ format!("Player Name: {}", test_result.player_name) }</p>
            </div>
        },
        Err(error_message) => html! {
            <div class="results-page">
                <h1>{ "Oops!" }</h1>
                <p>{ "It seems like something went wrong with displaying your test results." }</p>
                <p>{ error_message }</p>
                <p>{ "Maybe the code was incorrect, or perhaps our server just needs a coffee break. 🤷‍♂️ ☕" }</p>
            </div>
        },
    }
}
