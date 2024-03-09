use crate::pages::{About, Home, Navigation, Route};
use yew::prelude::*;
use yew_bootstrap::util::*;
use yew_router::prelude::*;

fn switch_main(route: Route) -> Html {
    match route {
        Route::About => html! {<About />},
        Route::Home => html! {<Home />}
    }
}

#[function_component]
pub fn App() -> Html {
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
