use crate::components::KonnektorCarousel;
use crate::model::{KonnektorDetail, Konnektoren};
use yew::prelude::*;
use yew_bootstrap::util::*;

#[function_component]
pub fn App() -> Html {
    let konnektoren = Konnektoren::default();
    let konnektoren = konnektoren.categories[0].details.clone();

    html! {
        <div>
            {include_cdn()}
            <h1>{"Konnektoren"}</h1>

            <KonnektorCarousel {konnektoren} />
            {include_cdn_js()}
        </div>
    }
}
