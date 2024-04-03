use super::TestChart;
use crate::model::{CategorizedTest, DetailTrait, TypeTrait};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TestStatisticsProps<T: TypeTrait, D: DetailTrait> {
    pub test: CategorizedTest<T, D>,
}

#[function_component(TestStatistics)]
pub fn test_statistics<T: TypeTrait + 'static, D: DetailTrait + 'static>(
    props: &TestStatisticsProps<T, D>,
) -> Html {
    let correct_answers = props
        .test
        .answers
        .iter()
        .filter(|record| {
            record.was_answered
                && record.correct_answer == record.user_answer.clone().unwrap_or_default()
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
        .items
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
                            .items
                            .get_detail_by_index(record.detail_index)
                            .unwrap()
                            == detail
                    }) {
                        return record.was_answered
                            && record.correct_answer
                                == record.user_answer.clone().unwrap_or_default();
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
        <div class="carousel-background-image"></div>
        <div class="test-statistics-container d-flex justify-content-center">
            <div>
                <h2>{"Test Statistics"}</h2>
                <p>{format!("Total Questions: {}", props.test.len())}</p>
                <div class="chart-container">
                    <TestChart<T, D> test={props.test.clone()} />
                </div>
                <p>{format!("Correct Answers: {}/{}", correct_answers, total_answered)}</p>
                <h3>{"Statistics by Category"}</h3>
                <ul>
                    {for stats_by_category.iter().map(|(category, correct, total)| {
                        html! { <li>{format!("{}: {}/{}", category, correct, total)}</li> }
                    })}
                </ul>
            </div>
        </div>
    </>
    }
}
