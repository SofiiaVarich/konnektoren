use crate::utils::translation::{flag, languages, LANGUAGE_KEY};
use gloo_storage::{LocalStorage, Storage};
use web_sys::HtmlSelectElement;
use yew::prelude::*;
use yew_i18n::use_translation;

#[function_component(SelectLanguage)]
pub fn select_language() -> Html {
    let mut i18n = use_translation();

    let selected_language =
        use_state(|| LocalStorage::get(LANGUAGE_KEY).unwrap_or_else(|_| "".to_string()));

    let _ = i18n.set_translation_language(&selected_language);

    let on_select_change = {
        let selected_language = selected_language.clone();
        Callback::from(move |e: Event| {
            let select = e.target_dyn_into::<HtmlSelectElement>();
            if let Some(select) = select {
                let value = select.value();
                selected_language.set(value.clone());
                let _ = LocalStorage::set(LANGUAGE_KEY, &value);
                // reload the page to apply the new language
                web_sys::window().unwrap().location().reload().unwrap();
            }
        })
    };

    html! {
        <div class="select-language">
            <p>
                { i18n.t("Please select a language from the dropdown.") }
                <select onchange={on_select_change} value={(*selected_language).clone()}>
                    <option value="" selected={selected_language.is_empty()} disabled=true>{ i18n.t("Select Language") }</option>
                    { for languages().iter().map(|&lang| html! {
                        <option value={lang} selected={lang == *selected_language}>{format!("{} {}", flag(lang), lang)}</option>
                    })}
                </select>
            </p>
        </div>
    }
}
