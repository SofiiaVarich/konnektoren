use crate::components::Footer;
use crate::utils::translation::LANGUAGE_KEY;
use gloo_storage::{LocalStorage, Storage};
use yew::prelude::*;
use yew_i18n::use_translation;

#[function_component(Survey)]
pub fn survey() -> Html {
    let i18n = {
        let mut i18n = use_translation();
        let selected_language =
            use_state(|| LocalStorage::get(LANGUAGE_KEY).unwrap_or_else(|_| "en".to_string()));
        let _ = i18n.set_translation_language(&selected_language);
        i18n
    };

    html! {
        <div class="survey-page">
        <div>
            <h1>{ i18n.t("Survey") }</h1>
            <iframe src="https://docs.google.com/forms/d/e/1FAIpQLSdfItM8tWzKHRdlXCwpQx_vjvw9N6qEjliDUKsTxZw4EKLJcw/viewform?embedded=true" width="640" height="640" frameborder="0" marginheight="0" marginwidth="0">
            {"Loading…"}
            </iframe>
        </div>
            <Footer />
        </div>
    }
}
