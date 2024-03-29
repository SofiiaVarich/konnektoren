use crate::model::{history::HISTORY_KEY, History};
use gloo_storage::{LocalStorage, Storage};
use yew::prelude::*;

#[function_component(HistoryPage)]
pub fn history_page() -> Html {
    let history = use_state(|| match LocalStorage::get::<History>(HISTORY_KEY) {
        Ok(stored_history) => stored_history,
        Err(_) => History::default(),
    });

    let longest_streak = history.longest_streak();
    let test_count = history.get_test_results().len();

    html! {
        <div class="history-page">
            <h1>{ "Learning History" }</h1>
            <p>{ format!("Longest streak: {} days in a row", longest_streak) }</p>
            <p>{ format!("Total tests taken: {}", test_count) }</p>
        </div>
    }
}
