use yew::prelude::*;

use crate::components::{PlayerInput, SoundPlayer};
use crate::model::{CategorizedTest, DetailTrait, TypeTrait};

#[derive(Properties, PartialEq)]
pub struct CongratulationsProps<T: TypeTrait, D: DetailTrait> {
    pub test: CategorizedTest<T, D>,
}

#[function_component(Congratulations)]
pub fn congratulations<T: TypeTrait, D: DetailTrait>(props: &CongratulationsProps<T, D>) -> Html {
    let correct_answers = props
        .test
        .answers
        .iter()
        .filter(|answer| answer.is_correct())
        .count();
    let total_answers = props.test.len();
    let performance = if total_answers > 0 {
        100.0 * correct_answers as f64 / total_answers as f64
    } else {
        0.0
    };

    html! {
        <div class="congratulations-container">
            <h2>{"Congratulations!"}</h2>
            <SoundPlayer sound_url="/fanfare-3-rpg.ogg" />
            <p>{format!("You have completed the test with a score of {:.1}% ({}/{})", performance, correct_answers, total_answers)}</p>
            {message::<T, D>(performance)}
            {player_input::<T>(total_answers, correct_answers, total_answers - correct_answers)}
        </div>
    }
}

fn player_input<T: TypeTrait>(
    total_questions: usize,
    correct_answers: usize,
    incorrect_answers: usize,
) -> Html {
    let test_type = T::get_t();

    html! {
        <PlayerInput test_type={test_type} total_questions={total_questions} correct_answers={correct_answers} incorrect_answers={incorrect_answers} />
    }
}

fn message<T: TypeTrait, D: DetailTrait>(performance: f64) -> Html {
    let message = if performance >= 90.0 {
        "Outstanding! You've mastered this topic with excellent results."
    } else if performance >= 75.0 {
        "Excellent job! You're well on your way to becoming an expert."
    } else if performance >= 60.0 {
        "Very good! Your hard work is paying off."
    } else if performance >= 45.0 {
        "Good job! Keep studying, and you'll see even more improvement."
    } else if performance >= 30.0 {
        "A solid attempt! Review the materials and try again for a better score."
    } else {
        "It looks like this topic is still a bit tough for you. Don't worry, with more practice, you'll get there!"
    };

    html! {
        <>
            <p>{format!("Based on your performance in the {} test: {}", T::get_type(), message)}</p>
        </>
    }
}
