use crate::components::{Carousel, Footer, TestSelector};
use crate::model::{AdjectiveDetail, KonnektorDetail, KonnektorType, PrepositionType};
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
            </>}
        } else {
            html! {
                <>
            <Carousel<PrepositionType, AdjectiveDetail> /></> }
        }
    };

    html! {
        <div>
            {include_cdn()}
            <h1 class="text-center">{"Konnektoren"}</h1>
           <TestSelector {show_konnektor_test} />
            {carousel}
            <Footer />
            {include_cdn_js()}
        </div>
    }
}
