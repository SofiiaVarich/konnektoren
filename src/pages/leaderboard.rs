use crate::model::Leaderboard;
use crate::route::Route;
use crate::utils::translation::LANGUAGE_KEY;
use gloo_storage::{LocalStorage, Storage};
use gloo_utils::format::JsValueSerdeExt;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use wasm_bindgen_futures::JsFuture;
use web_sys::Request;
use web_sys::RequestInit;
use web_sys::RequestMode;
use web_sys::Response;
use yew::prelude::*;
use yew_i18n::use_translation;
use yew_router::prelude::*;

#[function_component(LeaderboardPage)]
pub fn leaderboard_page() -> Html {
    let mut i18n = use_translation();

    let selected_language =
        use_state(|| LocalStorage::get(LANGUAGE_KEY).unwrap_or_else(|_| "en".to_string()));

    let _ = i18n.set_translation_language(&selected_language);

    let navigator = use_navigator().expect("No navigator");

    let leaderboard = use_state(Leaderboard::default);

    let leaderboard_clone = leaderboard.clone();
    use_effect(move || {
        let leaderboard = leaderboard_clone.clone();
        spawn_local(async move {
            if !leaderboard.get_tests().is_empty() {
                return;
            }

            let mut opts = RequestInit::new();
            opts.method("GET");
            opts.mode(RequestMode::Cors);

            let request = Request::new_with_str_and_init("/leaderboard.json", &opts).unwrap();

            let window = web_sys::window().unwrap();
            let resp_value = JsFuture::from(window.fetch_with_request(&request))
                .await
                .unwrap();
            let resp: Response = resp_value.dyn_into().unwrap();

            if !resp.ok() {
                log::error!("Response not ok: {:?}", resp);
                return;
            }

            match resp.json() {
                Ok(json_promise) => {
                    let json = JsFuture::from(json_promise).await.unwrap();
                    match json.into_serde::<Leaderboard>() {
                        Ok(leaderboard_json) => {
                            leaderboard.set(leaderboard_json);
                        }
                        Err(err) => {
                            log::error!("Failed to parse leaderboard: {:?}", err);
                        }
                    }
                }
                Err(err) => {
                    log::error!("Failed to parse leaderboard: {:?}", err);
                }
            }
        });

        || ()
    });

    let empty_message = match leaderboard.get_ranked().is_empty() {
        true => html! {
            <p>{ i18n.t("Loading...")}</p>
        },
        false => html! {},
    };

    html! {
        <div class="leaderboard-page">
            <h1>{ i18n.t("Leaderboard")}</h1>
            { empty_message }
            <ul>
                { for leaderboard.get_ranked().iter().map(|test| {
                    let navigator = navigator.clone();
                    let encoded = test.to_base64();
                    let onclick = {
                        let navigator = navigator.clone();
                        Callback::from(move |_| navigator.push(&Route::Results { code: encoded.clone() }))
                    };
                    html! {
                        <li {onclick}>
                            { format!("{}: {}% - {}", test.player_name, test.performance_percentage, test.date.format("%Y-%m-%d %H:%M:%S").to_string()) }
                        </li>
                    }
                })}
            </ul>
        </div>
    }
}
