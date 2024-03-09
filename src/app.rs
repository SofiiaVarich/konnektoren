use crate::components::{Carousel, Footer, Header, TestSelector};
use crate::model::{AdjectiveDetail, AdjectiveType, KonnektorDetail, KonnektorType};
use yew::prelude::*;
use yew_bootstrap::util::*;

#[function_component]
pub fn App() -> Html {
    let show_konnektor_test = use_state(|| true);

    let carousel = {
        if *show_konnektor_test {
            html! {
                    <>
                    <Carousel<KonnektorType, KonnektorDetail> />
                </>
            }
        } else {
            html! {
                    <>
                    <Carousel<AdjectiveType, AdjectiveDetail> />
                </>
            }
        }
    };

    html! {
        <div>
            {include_cdn()}
            <Header title={"Konnektoren".to_string()} img_src={"favicon.png".to_string()} />
           <TestSelector {show_konnektor_test} />
            {carousel}
            <Footer />
            {include_cdn_js()}
        </div>
    }
}
