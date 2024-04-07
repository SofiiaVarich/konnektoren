use crate::utils::translation::LANGUAGE_KEY;
use gloo_storage::{LocalStorage, Storage};
use yew::prelude::*;
use yew_i18n::use_translation;

#[function_component(Footer)]
pub fn footer() -> Html {
    let mut i18n = use_translation();

    let selected_language =
        use_state(|| LocalStorage::get(LANGUAGE_KEY).unwrap_or_else(|_| "en".to_string()));

    let _ = i18n.set_translation_language(&selected_language);

    html! {
        <footer class="footer">
            <div class="container text-center">
            <a id="wot-badge0" class="wot-badge" href="https://www.mywot.com/scorecard/konnektoren.help?wot_badge=0_white" target="_blank" >
                <div class="wot-logo"></div>
                <div class="wot-shield"></div>
                <p class="wot-secured">{"Verified Website"}</p>
                <div class="wot-vertical"></div>
                <p class="wot-report">{"See Report"}</p>
            </a>
                <p>{ i18n.t("The Konnektoren examples featured here are derived from the German DTB C1 Course.")}</p>
                <p>{ i18n.t("Special thanks to the educators and learners at the IFS Academy for their contributions to the course materials.")}</p>
            </div>
        </footer>
    }
}
