use crate::components::{Logo, SelectLanguageFlag};
use crate::utils::translation::LANGUAGE_KEY;
use gloo_storage::{LocalStorage, Storage};
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct HeaderProps {
    pub title: String,
    pub img_src: String,
}

#[function_component(Header)]
pub fn header(props: &HeaderProps) -> Html {
    let lang: Option<String> = LocalStorage::get(LANGUAGE_KEY).unwrap_or(None);

    let select_language = match lang {
        Some(_lang) => {
            html! {}
        }
        None => {
            html! {
                <SelectLanguageFlag />
            }
        }
    };
    html! {
        <div class="header text-center title-with-icon tour-welcome">
            <Logo img_src={props.img_src.clone()}/>
            <h1>{ &props.title }</h1>
            { select_language }
        </div>
    }
}
