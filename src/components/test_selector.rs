use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TestSelectorProps {
    pub show_konnektor_test: UseStateHandle<bool>,
}

#[function_component(TestSelector)]
pub fn test_selector(props: &TestSelectorProps) -> Html {
    let toggle_test = {
        let show_konnektor_test = props.show_konnektor_test.clone();
        Callback::from(move |_| show_konnektor_test.set(!*show_konnektor_test))
    };

    html! {
        <div class="test-selector text-center">
            <button onclick={toggle_test} class="toggle-test-btn">
                if *props.show_konnektor_test {
                    <><span class="large-font">{"Konnektoren"}</span><span class="small-font">{" / Adjektive mit Präpositionen"}</span></>
                } else {
                    <><span class="small-font">{"Konnektoren / "}</span><span class="large-font">{"Adjektive mit Präpositionen"}</span></>
                }
            </button>
        </div>
    }
}
