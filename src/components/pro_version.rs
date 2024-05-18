use gloo_utils::format::JsValueSerdeExt;
use serde_json::json;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::window;
use yew::prelude::*;

#[function_component(ProVersion)]
pub fn pro_version() -> Html {
    let show_message = use_state(|| false);

    let on_activate_click = {
        let show_message = show_message.clone();
        Callback::from(move |_| {
            // Trigger Google Analytics event
            if let Some(window) = window() {
                if let Some(gtag) = window.get("gtag") {
                    if let Ok(gtag) = gtag.dyn_into::<js_sys::Function>() {
                        let _ = gtag.call2(
                            &JsValue::NULL,
                            &JsValue::from("event"),
                            &JsValue::from_serde(&json!({
                                "send_to": "default",
                                "event_category": "pro_version",
                                "event_label": "Activate Pro Version Button",
                                "value": 1
                            }))
                            .unwrap(),
                        );
                    }
                }
            }
            show_message.set(true);
        })
    };

    html! {
        <div class="pro-version-container">
            <h1>{ "Pro Version" }</h1>
            <p>{ "Unlock more content and features with our Pro version. Enhance your learning experience with exclusive materials and tools." }</p>
            <button onclick={on_activate_click}>{ "Activate More Content" }</button>
            if *show_message {
                <div class="coming-soon">
                    <h2>{ "Coming Soon" }</h2>
                    <p>{ "The content is not ready yet but is coming soon and currently under construction. Stay tuned!" }</p>
                </div>
            }
        </div>
    }
}
