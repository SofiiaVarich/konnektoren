use crate::pages::Home;
use yew::prelude::*;
use yew_bootstrap::util::*;

#[function_component]
pub fn App() -> Html {
    html! {
        <div>
            {include_cdn()}
            <Home/>
            {include_cdn_js()}
        </div>
    }
}
