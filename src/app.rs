use crate::components::{Carousel, Footer, Header, TestSelector};
use crate::model::{
    AdjectiveDetail, AdjectiveType, KonnektorDetail, KonnektorType, TestType, VerbDetail, VerbType,
};
use yew::prelude::*;
use yew_bootstrap::util::*;

#[function_component]
pub fn App() -> Html {
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
    };

    html! {
        <div>
            {include_cdn()}
            <Header title={"Konnektoren".to_string()} img_src={"favicon.png".to_string()} />
           <TestSelector {test_type} />
            {carousel}
            <Footer />
            {include_cdn_js()}
        </div>
    }
}
