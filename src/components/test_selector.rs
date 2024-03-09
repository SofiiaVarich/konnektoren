use crate::model::TestType;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TestSelectorProps {
    pub test_type: UseStateHandle<TestType>,
}

#[function_component(TestSelector)]
pub fn test_selector(props: &TestSelectorProps) -> Html {
    let toggle_test = {
        let test_type = props.test_type.clone();
        Callback::from(move |_| {
            let next_type = match *test_type {
                TestType::Konnektoren => TestType::Adjectives,
                TestType::Adjectives => TestType::Verbs,
                TestType::Verbs => TestType::Konnektoren,
            };
            test_type.set(next_type);
        })
    };

    html! {
        <div class="test-selector text-center">
            <button onclick={toggle_test} class="toggle-test-btn">
                <span class={(*props.test_type == TestType::Konnektoren).then(|| "large-font").unwrap_or("small-font")}>
                    {TestType::Konnektoren.to_string()}
                </span>
                {" / "}
                <span class={(*props.test_type == TestType::Adjectives).then(|| "large-font").unwrap_or("small-font")}>
                    {TestType::Adjectives.to_string()}
                </span>
                {" / "}
                <span class={(*props.test_type == TestType::Verbs).then(|| "large-font").unwrap_or("small-font")}>
                    {TestType::Verbs.to_string()}
                </span>
            </button>
        </div>
    }
}
