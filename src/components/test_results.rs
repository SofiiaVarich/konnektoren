use crate::model::{CategorizedTest, DetailTrait, TypeTrait};

use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TestResultsProps<T: TypeTrait, D: DetailTrait> {
    pub test: CategorizedTest<T, D>,
}

#[function_component(TestResults)]
pub fn test_results<T: TypeTrait, D: DetailTrait>(props: &TestResultsProps<T, D>) -> Html {
    html! {
        <div class="test-results">
            <h3>{"Test Results"}</h3>
            <table>
                <thead>
                    <tr>
                        <th>{T::get_type()}</th>
                        <th>{"Your Answer"}</th>
                        <th>{"Correct Answer"}</th>
                        <th>{"Example"}</th>
                    </tr>
                </thead>
                <tbody>
                    {for props.test.answers.iter().filter(|answer| answer.was_answered).map(|answer| {
                        let detail = props.test.items.get_detail_by_index(answer.detail_index).unwrap();
                        let row_class = if answer.is_correct() { "correct-answer" } else { "incorrect-answer" };

                        html! {
                            <tr class={row_class}>
                                <td>{ &detail.get_detail() }</td>
                                <td>{ format!("{:?}", answer.user_answer.clone().unwrap_or(T::default())) }</td>
                                <td>{ format!("{:?}", answer.correct_answer) }</td>
                                <td>{ &detail.get_example() }</td>
                            </tr>
                        }
                    })}
                </tbody>
            </table>
        </div>
    }
}
