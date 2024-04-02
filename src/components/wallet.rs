use crate::utils::translation::LANGUAGE_KEY;
use gloo_storage::{LocalStorage, Storage};
use wasm_bindgen::prelude::*;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_i18n::use_translation;

#[wasm_bindgen(
    inline_js = "export function connectSolanaWallet() { return window.connectSolanaWallet(); }"
)]
extern "C" {
    async fn connectSolanaWallet() -> JsValue;
}

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

    let on_change = props.on_change.clone();
    let on_click = Callback::from(move |_| {
        let on_change = on_change.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let address = connectSolanaWallet().await;
            if !address.is_undefined() && !address.is_null() {
                let address_str: String = address.as_string().unwrap_or_default();
                web_sys::console::log_1(&format!("Wallet address: {}", address_str).into());
                on_change.emit(address_str);
            } else {
                web_sys::console::log_1(&"Failed to connect to the wallet.".into());
            }
        });
    });

    html! {
        <div class="wallet">
            <label for="account">{ i18n.t("Solana Account: ") }</label>
            <input id="account" type="text" value={props.address.clone()} oninput={on_input_change} placeholder="Solana Account Address" />
            <button onclick={on_click}>{ i18n.t("Connect Wallet") }</button>
       </div>
    }
}
