use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ProgressBarProps {
    pub value: usize,
    pub label: String,
}

#[function_component(ProgressBar)]
pub fn progress_bar(props: &ProgressBarProps) -> Html {
    let progress = props.value as f64;

    html! {
        <div class="progress-container">
            <div class="progress-bar" style={format!("width: {:.2}%;", progress)}>
                {props.label.clone()}
            </div>
        </div>
    }
}
