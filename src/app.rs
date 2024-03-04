use crate::components::{Footer, KonnektorCarousel};
use crate::model::Konnektoren;
use yew::prelude::*;
use yew_bootstrap::util::*;

#[function_component]
pub fn App() -> Html {
    let konnektoren = Konnektoren::default();

    html! {
        <div>
            {include_cdn()}
            <h1 class="text-center">{"Konnektoren"}</h1>
            <KonnektorCarousel {konnektoren} />
            <Footer />
            {include_cdn_js()}
        </div>
    }
}
