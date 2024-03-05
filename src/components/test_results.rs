use crate::model::{AnswerRecord, KonnektorTest, KonnektorType};

use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TestResultsProps {
    pub test: KonnektorTest,
}

#[function_component(TestResults)]
pub fn test_results(props: &TestResultsProps) -> Html {
    let incorrect_answers: Vec<&AnswerRecord> = props
        .test
        .answers
        .iter()
        .filter(|answer| !answer.is_correct() && answer.was_answered)
        .collect();

    html! {
        <>
            <h3>{"Test Results"}</h3>
            <table>
                <thead>
                    <tr>
                        <th>{"Konnektor"}</th>
                        <th>{"Your Answer"}</th>
                        <th>{"Correct Answer"}</th>
                        <th>{"Example"}</th>
                    </tr>
                </thead>
                <tbody>
                    {for incorrect_answers.iter().map(|answer| {

                        let detail = props.test.konnektoren.get_detail_by_index(answer.detail_index).unwrap();

                        html! {
                            <tr>
                                <td>{ &detail.konnektor }</td>
                                <td>{ format!("{:?}", answer.user_answer.clone().unwrap_or(KonnektorType::default())) }</td>
                                <td>{ format!("{:?}", answer.correct_answer) }</td>
                                <td>{ &detail.example }</td>
                            </tr>
                        }
                    })}
                </tbody>
            </table>
        </>
    }
}
