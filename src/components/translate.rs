use crate::utils::translation::LANGUAGE_KEY;
use gloo_storage::{LocalStorage, Storage};
use yew::prelude::*;
use yew_i18n::use_translation;

#[derive(Properties, PartialEq)]
pub struct TranslateProps {
    pub text: String,
}

#[function_component(Translate)]
pub fn translate(props: &TranslateProps) -> Html {
    let i18n = {
        let mut i18n = use_translation();
        let selected_language =
            use_state(|| LocalStorage::get(LANGUAGE_KEY).unwrap_or_else(|_| "en".to_string()));
        let _ = i18n.set_translation_language(&selected_language);
        i18n
    };

    let google_translate_url = format!(
        "https://translate.google.com/?sl=auto&tl=en&text={}",
        props.text
    );
    html! {
        <div class="translate-button tour-translate">
            <a href={google_translate_url} target="_blank" rel="noopener noreferrer">
                { i18n.t("Translate") }
            </a>
        </div>
    }
}
