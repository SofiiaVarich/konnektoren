use crate::components::{Carousel, Explanation, Footer, Header};
use crate::model::{KonnektorDetail, KonnektorType, TestType};
use konnektoren_yew::components::MusicComponent;
use yew::prelude::*;

#[function_component]
pub fn KonnektorenPage() -> Html {
    let carousel = html! {
        <Carousel<KonnektorType, KonnektorDetail> />
    };

    html! {
        <div>
            <MusicComponent url="/assets/background_main.wav" />
            <Header title={"Konnektoren".to_string()} img_src={"favicon.png".to_string()} />
            <Explanation test_type={TestType::Konnektoren} />
            {carousel}
            <Footer />
        </div>
    }
}
