use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct LogoProps {
    pub img_src: String,
}

#[function_component(Logo)]
pub fn logo(props: &LogoProps) -> Html {
    html! {
        <div class="logo">
            <img src={props.img_src.clone()} alt="Logo" />
        </div>
    }
}
