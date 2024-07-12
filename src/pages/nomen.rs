use crate::components::{Carousel, Explanation, Footer, Header};
use crate::model::{NomenDetail, NomenType, TestType};
use yew::prelude::*;

#[function_component]
pub fn NomenPage() -> Html {
    let carousel = html! {
        <Carousel<NomenType, NomenDetail> />
    };

    html! {
        <div>
            <MusicComponent url="/assets/background_main.wav" />
            <Header title={"Nomen-Verb-Verbindungen".to_string()} img_src={"favicon.png".to_string()} />
            <Explanation test_type={TestType::Nomen} />
            {carousel}
            <Footer />
        </div>
    }
}
