use crate::model::player::PLAYER_KEY;
use crate::model::Player;
use crate::model::TestResult;
use crate::model::TestType;
use gloo_storage::{LocalStorage, Storage};
use gloo_utils::format::JsValueSerdeExt;
use serde::{Deserialize, Serialize};
use urlencoding::encode;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use wasm_bindgen_futures::JsFuture;
use web_sys::MouseEvent;
use web_sys::{window, RequestInit, RequestMode};
use yew::prelude::*;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MintResponse {
    pub page: String,
}

fn nft_url(test_type: &TestType) -> String {
    match test_type {
        TestType::Konnektoren => "/assets/nft-konnektoren.png".to_string(),
        TestType::Adjectives => "/assets/nft-adjectives.png".to_string(),
        TestType::Verbs => "/assets/nft-verbs.png".to_string(),
        TestType::Nomen => "/assets/nft-nouns.png".to_string(),
    }
}

#[derive(Properties, PartialEq)]
pub struct MintNftProps {
    pub test_result: TestResult,
}

#[function_component(MintNFT)]
pub fn mint_nft(props: &MintNftProps) -> Html {
    let player =
        use_state(|| LocalStorage::get::<Player>(PLAYER_KEY).unwrap_or_else(|_| Player::default()));
    let is_loading = use_state(|| false);

    let page_link = use_state(|| None);
    let error_message = use_state(|| None::<String>);

    let test_result = props.test_result.clone();

    let receiver: String = player.account.clone().unwrap_or_default();

    let on_click = {
        let is_loading = is_loading.clone();
        is_loading.set(true);
        let page_link = page_link.clone();
        let error_message = error_message.clone();
        let test_result_clone = test_result.clone();
        let receiver = receiver.clone();

        Callback::from(move |_: MouseEvent| {
            let is_loading = is_loading.clone();
            if receiver.is_empty() {
                error_message.set(Some(
                    "Please set your Solana account address in the profile page.".to_string(),
                ));
                return;
            }

            let page_link = page_link.clone();
            let error_message = error_message.clone();
            let test_result = test_result_clone.clone();
            let code = encode(&test_result.to_base64()).to_string();
            let receiver = receiver.clone();
            spawn_local(async move {
                let window = window().expect("should have a Window");
                let mut opts = RequestInit::new();
                opts.method("GET");
                opts.mode(RequestMode::Cors);
                let request = web_sys::Request::new_with_str_and_init(
                    &format!("/mint?code={}&receiver={}", code, receiver),
                    &opts,
                )
                .expect("Failed to create request.");

                match JsFuture::from(window.fetch_with_request(&request)).await {
                    Ok(resp_value) => {
                        is_loading.set(false);
                        let resp: web_sys::Response = resp_value.dyn_into().unwrap();
                        if resp.ok() {
                            match resp.json() {
                                Ok(json) => {
                                    let json = match JsFuture::from(json).await {
                                        Ok(json) => json,
                                        Err(_) => {
                                            error_message
                                                .set(Some("Failed to parse response.".into()));
                                            return;
                                        }
                                    };
                                    match json.into_serde::<MintResponse>() {
                                        Ok(data) => page_link.set(Some(data.page)),
                                        Err(_) => error_message
                                            .set(Some("Failed to parse response.".into())),
                                    }
                                }
                                Err(_) => {
                                    error_message.set(Some("Failed to parse response.".into()))
                                }
                            }
                        } else {
                            error_message.set(Some("Request failed.".into()));
                        }
                    }
                    Err(_) => {
                        is_loading.set(false);
                        error_message.set(Some("Failed to fetch mint endpoint.".into()))
                    }
                }
            });
        })
    };

    let spinner = if *is_loading {
        html! {
            <div class="loader"></div>
        }
    } else {
        html! {}
    };

    let test_result = props.test_result.clone();

    let error_msg = match error_message.as_ref() {
        Some(msg) => {
            html! {
                <div class="mint-nft_error">
                    { msg }
                </div>
            }
        }
        None => {
            html! {}
        }
    };

    let nft = match page_link.as_ref() {
        Some(link) => {
            html! {
                <div class="mint-nft_title">
                <a href={link.clone()} target="_blank">
                    <img src={nft_url(&test_result.test_type)} tag="nft" />
                </a>
                </div>
            }
        }
        None => html! {
            <>
            <div class="mint-nft_title">
                { "Claim NFT" }
            </div>
            <div class="mint-nft_button">
                <button onclick={on_click}>
                    { "Mint" }
                </button>
            </div>
            </>
        },
    };

    match test_result.performance_percentage {
        0..=14 => {
            html! {}
        }
        _ => {
            html! {
                <div class="mint-nft">
                    { spinner }
                    { nft }
                    { error_msg }
                </div>
            }
        }
    }
}
