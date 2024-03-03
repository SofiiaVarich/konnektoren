use yew::prelude::*;
use yew_bootstrap::component::{Progress, ProgressBar};
use yew_bootstrap::util::Color;

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
        <Progress class="mb-3">
            <ProgressBar value={progress as i32} label={format!("Question {} of {}", props.current, props.total)} style={Color::Info} />
        </Progress>
    }
}
