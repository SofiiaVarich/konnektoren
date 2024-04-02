use crate::components::HistoryEntry;
use crate::model::{history::HISTORY_KEY, History};
use crate::utils::translation::LANGUAGE_KEY;
use gloo_storage::{LocalStorage, Storage};
use yew::prelude::*;
use yew_i18n::use_translation;

#[function_component(HistoryPage)]
pub fn history_page() -> Html {
    let mut i18n = use_translation();

    let selected_language =
        use_state(|| LocalStorage::get(LANGUAGE_KEY).unwrap_or_else(|_| "en".to_string()));

    let _ = i18n.set_translation_language(&selected_language);

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
          <div class="pages/history_bg.jpg"></div>
            <h1>{ i18n.t("Learning History") }</h1>
            <p>{ format!( "Longest streak: {} days in a row", longest_streak) }</p>
            <p>{ format!("Total tests taken: {}", test_count) }</p>
            <p>{ format!("Median performance: {:.2}%", median_performance) }</p>
            <h2>{ i18n.t("Tests Taken by Type") }</h2>
            <ul>
                { for count_by_test_type.iter().map(|(test_type, &count)| {
                    html! { <li>{ format!("{:?}: {}", test_type, count) }</li> }
                })}
            </ul>
            <h2>{ i18n.t("Average Performance by Test Type") }</h2>
            <ul>
                { for average_performance_by_test_type.iter().map(|(test_type, &avg_perf)| {
                    html! { <li>{ format!("{:?}: {:.2}%", test_type, avg_perf) }</li> }
                })}
            </ul>
            <h2>{ i18n.t("Test History")}</h2>
            <div class="history-entries">
                { for history.get_test_results().iter().map(|test_result| {
                    html! { <HistoryEntry test_result={test_result.clone()} /> }
                })}
            </div>
        </div>
    }
}
