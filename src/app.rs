use yew::prelude::*;
use yew_bootstrap::util::*;

#[function_component]
pub fn App() -> Html {
    html! {
        <div>
            {include_cdn()}
            <h1>{"Konnektoren"}</h1>
            {include_cdn_js()}
        </div>
    }
}
