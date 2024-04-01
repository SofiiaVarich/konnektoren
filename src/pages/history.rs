use crate::components::HistoryEntry;
use crate::model::{history::HISTORY_KEY, History};
use gloo_storage::{LocalStorage, Storage};
use yew::prelude::*;

#[function_component(HistoryPage)]
pub fn history_page() -> Html {
    let history = use_state(|| match LocalStorage::get::<History>(HISTORY_KEY) {
        Ok(stored_history) => stored_history,
        Err(_) => History::new(),
    });

    let longest_streak = history.longest_streak();
    let test_count = history.get_test_results().len();
    let median_performance = history.median_performance();
    let count_by_test_type = history.count_by_test_type();
    let average_performance_by_test_type = history.average_performance_by_test_type();

    html! {
        <div class="history-page">
            <h1>{ "Learning History" }</h1>
            <p>{ format!("Longest streak: {} days in a row", longest_streak) }</p>
            <p>{ format!("Total tests taken: {}", test_count) }</p>
            <p>{ format!("Median performance: {:.2}%", median_performance) }</p>
            <h2>{ "Tests Taken by Type" }</h2>
            <ul>
                { for count_by_test_type.iter().map(|(test_type, &count)| {
                    html! { <li>{ format!("{:?}: {}", test_type, count) }</li> }
                })}
            </ul>
            <h2>{ "Average Performance by Test Type" }</h2>
            <ul>
                { for average_performance_by_test_type.iter().map(|(test_type, &avg_perf)| {
                    html! { <li>{ format!("{:?}: {:.2}%", test_type, avg_perf) }</li> }
                })}
            </ul>
            <h2>{ "Test History" }</h2>
            <div class="history-entries">
                { for history.get_test_results().iter().map(|test_result| {
                    html! { <HistoryEntry test_result={test_result.clone()} /> }
                })}
            </div>
        </div>
    }
}
