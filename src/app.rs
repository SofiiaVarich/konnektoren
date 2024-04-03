use crate::{
    components::{MainMenu, Navigation},
    pages::{
        About, AdjectivesPage, ConfigPage, HistoryPage, Home, KonnektorenPage, LanguagePage,
        LeaderboardPage, ProfilePage, Results, VerbsPage,
    },
    route::Route,
    utils::translation::{languages, translations, LANGUAGE_KEY},
};
use gloo_storage::{LocalStorage, Storage};
use wasm_bindgen::JsValue;
use yew::prelude::*;
use yew_i18n::I18nProvider;
use yew_router::prelude::*;

fn switch_main(route: Route) -> Html {
    let supported_languages = languages();
    let translations = translations();

    let lang: Option<String> = LocalStorage::get(LANGUAGE_KEY).unwrap_or(None);

    match lang {
        Some(_lang) => {
            let route = match route {
                Route::About => html! {<About /> },
                Route::Home => html! {<Home />},
                Route::Konnektoren => html! {<KonnektorenPage />},
                Route::Adjectives => html! {<AdjectivesPage />},
                Route::Verbs => html! {<VerbsPage />},
                Route::Results { code } => html! {<Results { code } />},
                Route::Config => html! {<ConfigPage />},
                Route::Profile => html! {<ProfilePage />},
                Route::History => html! {<HistoryPage />},
                Route::Leaderboard => html! {<LeaderboardPage />},
            };

            html! {
                <I18nProvider supported_languages={supported_languages} translations={translations} >
                    {route}
                </I18nProvider>
            }
        }
        None => {
            html! {
                <I18nProvider supported_languages={supported_languages} translations={translations} >
                    <LanguagePage />
                </I18nProvider>
            }
        }
    }
}

pub struct App;

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        redirect_if_needed();
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div>
            <BrowserRouter>
                    <Navigation />
                    <MainMenu />
                    <Switch<Route> render={switch_main} />
            </BrowserRouter>
        </div>
        }
    }
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
