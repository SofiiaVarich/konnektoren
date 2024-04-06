use crate::components::Logo;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct HeaderProps {
    pub title: String,
    pub img_src: String,
}

#[function_component(Header)]
pub fn header(props: &HeaderProps) -> Html {
    html! {
        <div class="text-center title-with-icon">
                <Logo img_src={props.img_src.clone()}/>
            <h1>{ &props.title }</h1>
        </div>
    }
}