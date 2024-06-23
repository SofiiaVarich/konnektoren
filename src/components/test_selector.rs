use crate::model::TestType;
use strum::IntoEnumIterator;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TestSelectorProps {
    pub test_type: UseStateHandle<TestType>,
}

#[function_component(TestSelector)]
pub fn test_selector(props: &TestSelectorProps) -> Html {
    let set_test_type = {
        let test_type = props.test_type.clone();
        move |type_: TestType| {
            test_type.set(type_);
        }
    };

    html! {
        <div class="test-selector text-center tour-test">
            { for TestType::iter().map(|type_| {
                let is_active = *props.test_type == type_;
                let onclick = {
                    let set_test_type = set_test_type.clone();
                    Callback::from(move |_| set_test_type(type_))
                };
                html! {
                    <button
                        {onclick}
                        class={classes!("test-type-button", is_active.then_some("active"))}
                    >
                        { type_.to_string() }
                    </button>
                }
            })}
        </div>
    }
}
