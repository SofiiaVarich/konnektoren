use crate::components::{Carousel, Explanation, Footer, Header};
use crate::model::{TestType, VerbDetail, VerbType};
use yew::prelude::*;

#[function_component]
pub fn VerbsPage() -> Html {
    let carousel = html! {
        <Carousel<VerbType, VerbDetail> />
    };

    html! {
        <div>
            <Header title={"Verben".to_string()} img_src={"favicon.png".to_string()} />
            <Explanation test_type={TestType::Verbs} />
            {carousel}
            <Footer />
        </div>
    }
}
