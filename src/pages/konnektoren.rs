use crate::components::{Carousel, Footer, Header};
use crate::model::{KonnektorDetail, KonnektorType};
use yew::prelude::*;

#[function_component]
pub fn KonnektorenPage() -> Html {
    let carousel = html! {
        <Carousel<KonnektorType, KonnektorDetail> />
    };

    html! {
        <div>
            <Header title={"Konnektoren".to_string()} img_src={"favicon.png".to_string()} />
            {carousel}
            <Footer />
        </div>
    }
}
