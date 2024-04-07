use crate::components::Logo;
use crate::route::Route;
use crate::utils::translation::LANGUAGE_KEY;
use gloo_storage::{LocalStorage, Storage as _};
use yew::prelude::*;
use yew_i18n::use_translation;
use yew_router::prelude::*;

#[function_component]
pub fn Navigation() -> Html {
    let i18n = {
        let mut i18n = use_translation();
        let selected_language =
            use_state(|| LocalStorage::get(LANGUAGE_KEY).unwrap_or_else(|_| "en".to_string()));
        let _ = i18n.set_translation_language(&selected_language);
        i18n
    };
    html! {
        <div class="navigation-wrapper">
            <div class="navigation">
                <nav>
                    <Link<Route> to={Route::Konnektoren}>{ i18n.t("Konnektoren") }</Link<Route>>
                    <Link<Route> to={Route::Adjectives}>{ i18n.t("Adjektive") }</Link<Route>>
                    <Link<Route> to={Route::Home}><Logo img_src={"/favicon.png".to_string()} /></Link<Route>>
                    <Link<Route> to={Route::Verbs}>{ i18n.t("Verben") }</Link<Route>>
                    <Link<Route> to={Route::Leaderboard}>{ i18n.t("Leaderboard") }</Link<Route>>
                </nav>
            </div>
        </div>
    }
}
