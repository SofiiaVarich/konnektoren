use crate::components::{Carousel, Explanation, Footer, Header};
use crate::model::{AdjectiveDetail, AdjectiveType, TestType};
use konnektoren_yew::components::MusicComponent;
use yew::prelude::*;

#[function_component]
pub fn AdjectivesPage() -> Html {
    let carousel = html! {
        <Carousel<AdjectiveType, AdjectiveDetail> />
    };

    html! {
        <div>
            <MusicComponent url="/assets/background_main.wav" />
            <Header title={"Adjektive".to_string()} img_src={"favicon.png".to_string()} />
            <Explanation test_type={TestType::Adjectives} />
            {carousel}
            <Footer />
        </div>
    }
}
