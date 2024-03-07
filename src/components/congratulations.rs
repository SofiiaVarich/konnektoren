use crate::model::KonnektorTest;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct CongratulationsProps {
    pub test: KonnektorTest,
}

#[function_component(Congratulations)]
pub fn congratulations(props: &CongratulationsProps) -> Html {
    let correct_answers = props
        .test
        .answers
        .iter()
        .filter(|answer| answer.is_correct())
        .count();
    let total_answers = props
        .test
        .answers
        .iter()
        .filter(|answer| answer.was_answered)
        .count();
    let performance = if total_answers > 0 {
        100.0 * correct_answers as f64 / total_answers as f64
    } else {
        0.0
    };

    html! {
        <div class="congratulations-container">
            <h2>{"Congratulations!"}</h2>
            <p>{format!("You have completed the test with a score of {:.1}% ({}/{})", performance, correct_answers, total_answers)}</p>
            <p>{if performance >= 80.0 {
                "Fantastic job! You have a strong understanding of Konnektoren."
            } else if performance >= 50.0 {
                "Good effort! With a bit more practice, you'll master Konnektoren."
            } else {
                "Keep practicing! Konnektoren can be challenging, but you're making progress."
            }}</p>
        </div>
    }
}
