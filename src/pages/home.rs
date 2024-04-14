use crate::components::{Carousel, Header, SlideShow, TestSelector};
use crate::model::{
    AdjectiveDetail, AdjectiveType, KonnektorDetail, KonnektorType, NomenDetail, NomenType,
    TestType, VerbDetail, VerbType,
};
use yew::prelude::*;

#[function_component]
pub fn Home() -> Html {
    let test_type = use_state(|| TestType::Konnektoren);

    let carousel = match *test_type {
        TestType::Konnektoren => html! {
            <Carousel<KonnektorType, KonnektorDetail> />
        },
        TestType::Adjectives => html! {
            <Carousel<AdjectiveType, AdjectiveDetail> />
        },
        TestType::Verbs => html! {
            <Carousel<VerbType, VerbDetail> />
        },
        TestType::Nomen => html! {
            <Carousel<NomenType, NomenDetail> />
        },
    };

    html! {
        <div>
            <Header title={"Konnektoren".to_string()} img_src={"/favicon.png".to_string()} />
            <SlideShow urls={vec!["/slides/1.png".to_string(), "/slides/3.png".to_string(),
                "/slides/6.png".to_string(), "/slides/7.png".to_string(),
                "/slides/8.png".to_string(), "/slides/9.png".to_string()]} />
            <TestSelector {test_type} />
            {carousel}
        </div>
    }
}
