use super::TestChart;
use crate::model::KonnektorTest;
use crate::model::KonnektorType;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TestStatisticsProps {
    pub test: KonnektorTest,
}

#[function_component(TestStatistics)]
pub fn test_statistics(props: &TestStatisticsProps) -> Html {
    let correct_answers = props
        .test
        .answers
        .iter()
        .filter(|record| {
            record.was_answered
                && record.correct_answer
                    == <std::option::Option<KonnektorType> as Clone>::clone(&record.user_answer)
                        .unwrap_or_default()
        })
        .count();

    let total_answered = props
        .test
        .answers
        .iter()
        .filter(|record| record.was_answered)
        .count();

    let stats_by_category = props
        .test
        .konnektoren
        .categories
        .iter()
        .map(|category| {
            let total_in_category = category.details.len();
            let correct_in_category = category
                .details
                .iter()
                .filter(|detail| {
                    if let Some(record) = props.test.answers.iter().find(|record| {
                        &props
                            .test
                            .konnektoren
                            .get_detail_by_index(record.detail_index)
                            .unwrap()
                            == detail
                    }) {
                        return record.was_answered
                            && record.correct_answer
                                == <std::option::Option<KonnektorType> as Clone>::clone(
                                    &record.user_answer,
                                )
                                .unwrap_or_default();
                    }
                    false
                })
                .count();

            (
                category.category.clone(),
                correct_in_category,
                total_in_category,
            )
        })
        .collect::<Vec<_>>();

    html! {
        <>
            <h2>{"Test Statistics"}</h2>
            <p>{format!("Total Questions: {}", props.test.len())}</p>
            <TestChart test={props.test.clone()} />
            <p>{format!("Correct Answers: {}/{}", correct_answers, total_answered)}</p>
            <h3>{"Statistics by Category"}</h3>
            <ul>
                {for stats_by_category.iter().map(|(category, correct, total)| {
                    html! { <li>{format!("{}: {}/{}", category, correct, total)}</li> }
                })}
            </ul>
        </>
    }
}
