use crate::utils::translation::{flag, languages, LANGUAGE_KEY};
use gloo_storage::{LocalStorage, Storage};
use yew::prelude::*;
use yew_i18n::use_translation;

#[function_component(SelectLanguageFlag)]
pub fn select_language_flag() -> Html {
    let mut i18n = use_translation();

    let selected_language =
        use_state(|| LocalStorage::get(LANGUAGE_KEY).unwrap_or_else(|_| "en".to_string()));
    let _ = i18n.set_translation_language(&*selected_language);

    let on_flag_click = {
        let selected_language = selected_language.clone();
        move |lang: String| {
            let onclick = {
                let selected_language = selected_language.clone();
                Callback::from(move |_| {
                    selected_language.set(lang.clone());
                    let _ = LocalStorage::set(LANGUAGE_KEY, &lang);
                    web_sys::window().unwrap().location().reload().unwrap();
                })
            };
            onclick
        }
    };

    html! {
        <div class="select-language tour-language">
            <p>
                { i18n.t("Please select a language:") }
                <div class="language-flags">
                    { for languages().iter().map(|&lang| html! {
                        <span onclick={on_flag_click(lang.to_string())} class={if *selected_language == lang { "selected" } else { "" }}>
                            { format!("{} {}", flag(lang), lang) }
                        </span>
                    })}
                </div>
            </p>
        </div>
    }
}
