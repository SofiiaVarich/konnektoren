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
            <img src={props.img_src.clone()} alt="Icon" />
            <h1>{ &props.title }</h1>
        </div>
    }
}
