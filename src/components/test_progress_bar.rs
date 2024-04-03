use crate::components::ProgressBar;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ProgressBarProps {
    pub current: usize,
    pub total: usize,
}

#[function_component(TestProgressBar)]
pub fn test_progress_bar(props: &ProgressBarProps) -> Html {
    let progress = if props.total > 0 {
        100 * props.current / props.total
    } else {
        0
    };

    html! {
        <div class="fire-gradient-progress-bar">
            <ProgressBar value={progress} label={format!("Question {} of {}", props.current + 1, props.total)} />
        </div>
    }
}
