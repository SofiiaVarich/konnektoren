use crate::{
    pages::{About, AdjectivesPage, Home, KonnektorenPage, Navigation, Results, Route, VerbsPage},
    utils::translation::{languages, translations},
};
use wasm_bindgen::JsValue;
use yew::prelude::*;
use yew_bootstrap::util::*;
use yew_i18n::I18nProvider;
use yew_router::prelude::*;

fn switch_main(route: Route) -> Html {
    let supported_languages = languages();
    let translations = translations();

    let route = match route {
        Route::About => html! {<About /> },
        Route::Home => html! {<Home />},
        Route::Konnektoren => html! {<KonnektorenPage />},
        Route::Adjectives => html! {<AdjectivesPage />},
        Route::Verbs => html! {<VerbsPage />},
        Route::Results { code } => html! {<Results { code } />},
    };

    html! {
        <I18nProvider supported_languages={supported_languages} translations={translations} >
            {route}
        </I18nProvider>
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
            {include_cdn()}
            <BrowserRouter>
                    <Navigation />
                    <Switch<Route> render={switch_main} />
            </BrowserRouter>
            {include_cdn_js()}
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
