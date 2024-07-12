use crate::components::{Carousel, Explanation, Footer, Header};
use crate::model::{TestType, VerbDetail, VerbType};
use konnektoren_yew::components::MusicComponent;
use yew::prelude::*;

#[function_component]
pub fn VerbsPage() -> Html {
    let carousel = html! {
        <Carousel<VerbType, VerbDetail> />
    };

    html! {
        <div>
            <MusicComponent url="/assets/background_main.wav" />
            <Header title={"Verben".to_string()} img_src={"favicon.png".to_string()} />
            <Explanation test_type={TestType::Verbs} />
            {carousel}
            <Footer />
        </div>
    }
}
