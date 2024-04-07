use crate::components::{Footer, SelectLanguage};
use crate::utils::translation::LANGUAGE_KEY;
use gloo_storage::{LocalStorage, Storage};
use yew::prelude::*;
use yew_i18n::use_translation;

#[function_component(ConfigPage)]
pub fn config_page() -> Html {
    let i18n = {
        let mut i18n = use_translation();
        let selected_language =
            use_state(|| LocalStorage::get(LANGUAGE_KEY).unwrap_or_else(|_| "en".to_string()));
        let _ = i18n.set_translation_language(&selected_language);
        i18n
    };

    html! {
    <div class="navigation-wrapper">
                <div class="fluffy-ball">
                 <div class="eye left-eye"></div>
                 <div class="eye right-eye"></div>
                </div>
            <div class="config-page">
            <h1>{ i18n.t("Config") }</h1>
            <SelectLanguage />
            <Footer />
        </div>
        </div>
    }
}