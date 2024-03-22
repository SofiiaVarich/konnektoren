use crate::components::{Carousel, Header, TestSelector};
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
           <TestSelector {test_type} />
            {carousel}
        </div>
    }
}
