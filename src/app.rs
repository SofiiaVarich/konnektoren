use crate::{
    components::{MainMenu, Navigation},
    pages::{
        About, AdjectivesPage, ConfigPage, HistoryPage, Home, KonnektorenPage, LeaderboardPage,
        ProfilePage, Results, Survey, VerbsPage,
    },
    route::Route,
    utils::translation::{languages, supported_language, translations, LANGUAGE_KEY},
};
use gloo_storage::{LocalStorage, Storage as _};
use wasm_bindgen::JsValue;
use yew::prelude::*;
use yew_i18n::I18nProvider;
use yew_router::prelude::*;

fn switch_main(route: Route) -> Html {
    match route {
        Route::About => html! {<About /> },
        Route::Home => html! {<Home />},
        Route::Konnektoren => html! {<KonnektorenPage />},
        Route::Adjectives => html! {<AdjectivesPage />},
        Route::Verbs => html! {<VerbsPage />},
        Route::Results { code } => html! {<Results { code } />},
        Route::Config => html! {<ConfigPage />},
        Route::Profile => html! {<ProfilePage />},
        Route::History => html! {<HistoryPage />},
        Route::Survey => html! {<Survey />},
        Route::Leaderboard => html! {<LeaderboardPage />},
    }
}

pub struct App;

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        update_language();
        redirect_if_needed();
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let supported_languages = languages();
        let translations = translations();
        html! {
        <I18nProvider supported_languages={supported_languages} translations={translations} >
            <BrowserRouter>
                    <Navigation />
                    <MainMenu />
                    <Switch<Route> render={switch_main} />
            </BrowserRouter>
        </I18nProvider>
        }
    }
}

fn update_language() {
    let window = web_sys::window().expect("no global `window` exists");
    let location = window.location();
    let mut query = location
        .search()
        .expect("couldn't retrieve the query string");

    if query.starts_with('?') {
        query = query.trim_start_matches('?').to_string();
    }

    let lang = if !query.is_empty() {
        query
            .split('&')
            .find(|part| part.starts_with("lang="))
            .and_then(|lang_part| lang_part.split('=').nth(1))
    } else {
        None
    };
    let lang = supported_language(lang);

    match lang {
        Some(lang) => {
            let _ = LocalStorage::set(LANGUAGE_KEY, &lang);
        }
        _ => {}
    };
}

fn redirect_if_needed() {
    let window = web_sys::window().expect("no global `window` exists");
    let location = window.location();
    let query = location
        .search()
        .expect("couldn't retrieve the query string");

    if query.contains("page=about") {
        let history = window.history().expect("couldn't get history");
        history
            .push_state_with_url(&JsValue::NULL, "", Some("/about"))
            .expect("could not push state");
    }

    if query.contains("page=konnektoren") {
        let history = window.history().expect("couldn't get history");
        history
            .push_state_with_url(&JsValue::NULL, "", Some("/konnektoren"))
            .expect("could not push state");
    }

    if query.contains("page=adjectives") {
        let history = window.history().expect("couldn't get history");
        history
            .push_state_with_url(&JsValue::NULL, "", Some("/adjectives"))
            .expect("could not push state");
    }

    if query.contains("page=verbs") {
        let history = window.history().expect("couldn't get history");
        history
            .push_state_with_url(&JsValue::NULL, "", Some("/verbs"))
            .expect("could not push state");
    }

    if query.contains("page=leaderboard") {
        let history = window.history().expect("couldn't get history");
        history
            .push_state_with_url(&JsValue::NULL, "", Some("/leaderboard"))
            .expect("could not push state");
    }

    if query.contains("page=profile") {
        let history = window.history().expect("couldn't get history");
        history
            .push_state_with_url(&JsValue::NULL, "", Some("/profile"))
            .expect("could not push state");
    }

    if query.contains("page=history") {
        let history = window.history().expect("couldn't get history");
        history
            .push_state_with_url(&JsValue::NULL, "", Some("/history"))
            .expect("could not push state");
    }

    if query.contains("page=config") {
        let history = window.history().expect("couldn't get history");
        history
            .push_state_with_url(&JsValue::NULL, "", Some("/config"))
            .expect("could not push state");
    }

    if query.contains("page=results") {
        let code = query
            .split('&')
            .find(|part| part.starts_with("code="))
            .and_then(|code_part| code_part.split('=').nth(1))
            .unwrap_or("");

        let history = window.history().expect("couldn't get history");
        let new_url = format!("/results/{}", code);
        history
            .push_state_with_url(&JsValue::NULL, "", Some(&new_url))
            .expect("could not push state");
    }
}
