use crate::pages::{About, Home, Navigation, Results, Route};
use wasm_bindgen::JsValue;
use yew::prelude::*;
use yew_bootstrap::util::*;
use yew_router::prelude::*;

fn switch_main(route: Route) -> Html {
    match route {
        Route::About => html! {<About />},
        Route::Home => html! {<Home />},
        Route::Results { code } => html! {<Results { code } />},
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
