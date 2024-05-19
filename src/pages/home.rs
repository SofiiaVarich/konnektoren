use crate::components::{Carousel, Header, SlideShow, TestSelector};
use crate::model::{
    AdjectiveDetail, AdjectiveType, KonnektorDetail, KonnektorType, NomenDetail, NomenType,
    TestType, VerbDetail, VerbType, VorgangspassivDetail, VorgangspassivType,
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
        TestType::Vorgangspassiv => html! {
            <Carousel<VorgangspassivType, VorgangspassivDetail> />
        },
    };

    html! {
        <div>
            <Header title={"Konnektoren".to_string()} img_src={"/favicon.png".to_string()} />
            <SlideShow urls={vec!["/slides/1s.png".to_string(), "/slides/3s.png".to_string(),
                "/slides/6m.png".to_string(), "/slides/7m.png".to_string(),
                "/slides/8m.png".to_string(), "/slides/9s.png".to_string()]} />
            <TestSelector {test_type} />
            {carousel}
        </div>
    }
}
