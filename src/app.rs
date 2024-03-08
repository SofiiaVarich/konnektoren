use crate::components::{Footer, KonnektorCarousel};
use crate::model::{KonnektorDetail, KonnektorType};
use yew::prelude::*;
use yew_bootstrap::util::*;

#[function_component]
pub fn App() -> Html {
    html! {
        <div>
            {include_cdn()}
            <h1 class="text-center">{"Konnektoren"}</h1>
            <KonnektorCarousel<KonnektorType, KonnektorDetail>/>
            <Footer />
            {include_cdn_js()}
        </div>
    }
}
