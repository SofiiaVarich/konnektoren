use crate::utils::translation::LANGUAGE_KEY;
use gloo_storage::{LocalStorage, Storage};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_i18n::use_translation;

#[derive(Properties, PartialEq)]
pub struct WalletProps {
    pub address: String,
    pub on_change: Callback<String>,
}

#[function_component(Wallet)]
pub fn wallet(props: &WalletProps) -> Html {
    let mut i18n = use_translation();
    let selected_language =
        use_state(|| LocalStorage::get(LANGUAGE_KEY).unwrap_or_else(|_| "en".to_string()));
    let _ = i18n.set_translation_language(&selected_language);

    let on_input_change = {
        let on_change = props.on_change.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            on_change.emit(input.value());
        })
    };

    html! {
        <div class="wallet">
            <label for="account">{ i18n.t("Solana Account: ") }</label>
            <input id="account" type="text" value={props.address.clone()} oninput={on_input_change} placeholder="Solana Account Address" />
            <button>{ i18n.t("Connect Wallet") }</button>
       </div>
    }
}
