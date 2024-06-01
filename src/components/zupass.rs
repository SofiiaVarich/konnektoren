use crate::model::TestResult;
use wasm_bindgen::prelude::*;
use yew::prelude::*;

#[wasm_bindgen(
    inline_js = "
    export function generateUrl(data) {
        return pcdpassportInterface.constructPassportPcdProveAndAddRequestUrl(data);
    }
    "
)]
extern "C" {
    #[wasm_bindgen(js_name = generateUrl)]
    async fn generate_url(data: &str) -> JsValue;
}

#[derive(Properties, PartialEq)]
pub struct ZupassProps {
    pub test_result: TestResult,
}

#[function_component(Zupass)]
pub fn zupass(props: &ZupassProps) -> Html {
    let json_data = serde_json::to_string(&props.test_result).unwrap();

    let on_click = Callback::from(move |_|{
        let json_data = json_data.clone();
        wasm_bindgen_futures::spawn_local(async move {
            let url = generate_url(&json_data).await;
            if !url.is_undefined() && !url.is_null() {
                let url_str: String = url.as_string().unwrap_or_default();
                web_sys::console::log_1(&format!("Zupass URL: {}", url_str).into());
            } else {
                web_sys::console::log_1(&"Failed to get Zupass Url.".into());
            }
        });
    });

    html! {
        <div class="zupass">
            <h1>{"Zupass"}</h1>
            <button onclick={on_click}>{"Get your Certificate on Zupass"}</button>
        </div>
    }
}