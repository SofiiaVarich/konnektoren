use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TelegramLinkProps {
    pub username: String,
    pub url: String,
}

#[function_component(TelegramLink)]
pub fn telegram_link(props: &TelegramLinkProps) -> Html {
    html! {
        <div class="telegram-link">
            <a href={props.url.clone()} target="_blank" rel="noopener noreferrer">
                {format!("@{}", props.username)}
            </a>
        </div>
    }
}
