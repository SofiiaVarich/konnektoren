use crate::components::Logo;
use crate::route::Route;
use crate::utils::translation::LANGUAGE_KEY;
use gloo_storage::{LocalStorage, Storage as _};
use yew::prelude::*;
use yew_i18n::use_translation;
use yew_router::prelude::*;

#[function_component(MainMenu)]
pub fn main_menu() -> Html {
    let i18n = {
        let mut i18n = use_translation();
        let selected_language =
            use_state(|| LocalStorage::get(LANGUAGE_KEY).unwrap_or_else(|_| "en".to_string()));
        let _ = i18n.set_translation_language(&selected_language);
        i18n
    };

    let is_menu_open = use_state(|| false);

    let toggle_menu = {
        let is_menu_open = is_menu_open.clone();
        Callback::from(move |_| {
            is_menu_open.set(!*is_menu_open);
        })
    };

    let menu_class = if *is_menu_open {
        "menu-items active"
    } else {
        "menu-items"
    };

    html! {
        <div class="main-menu" onclick={toggle_menu.clone()}>
            <button class="burger-menu" aria-expanded={is_menu_open.to_string()} onclick={toggle_menu}>
                {"☰"}
            </button>

            if *is_menu_open {
                <div class={menu_class}>
                <br />
                    <Link<Route> to={Route::Home}>{html!{<Logo img_src={"/favicon.png".to_string()} />}}</Link<Route>>
                    <Link<Route> to={Route::Profile}>{ i18n.t("Profile") }</Link<Route>>
                    <Link<Route> to={Route::History}>{ i18n.t("History") }</Link<Route>>
                    <Link<Route> to={Route::Leaderboard}>{ i18n.t("Leaderboard") }</Link<Route>>
                    <Link<Route> to={Route::About}>{ i18n.t("About") }</Link<Route>>
                    <Link<Route> to={Route::Config}>{ i18n.t("Config") }</Link<Route>>
                </div>
            }
        </div>
    }
}
