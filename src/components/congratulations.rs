use crate::route::Route;
use crate::utils::translation::LANGUAGE_KEY;
use gloo_storage::{LocalStorage, Storage};
use yew::prelude::*;
use yew_i18n::use_translation;
use yew_router::prelude::*;

use crate::components::TestResults;
use crate::components::{PlayerInput, SoundPlayer};
use crate::model::{CategorizedTest, DetailTrait, TestResult, TypeTrait};
#[derive(Properties, PartialEq)]
pub struct CongratulationsProps<T: TypeTrait, D: DetailTrait> {
    pub test: CategorizedTest<T, D>,
}

#[function_component(Congratulations)]
pub fn congratulations<T: TypeTrait + 'static, D: DetailTrait + 'static>(
    props: &CongratulationsProps<T, D>,
) -> Html {
    let mut i18n = use_translation();
    let navigator = use_navigator().expect("No navigator");

    let test_result: UseStateHandle<Option<TestResult>> = use_state(|| None);

    let selected_language =
        use_state(|| LocalStorage::get(LANGUAGE_KEY).unwrap_or_else(|_| "en".to_string()));

    let _ = i18n.set_translation_language(&selected_language);

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

    let on_generate = {
        let result = test_result.clone();
        Callback::from(move |test_result| {
            result.set(Some(test_result));
        })
    };

    let certificate_part = match &props.test.example_showed {
        true => html! {
            <div>
                <h3>{ i18n.t("Certificate") }</h3>
                <p>{ i18n.t("If you can make the test without help of the examples, you can earn a certificate.") }</p>
                <p>{ i18n.t("If you can make the test with a performance of 80% and more, you can get a nft.") }</p>
                <TestResults<T,D> test={props.test.clone()} />
            </div>
        },
        false => match (*test_result).clone() {
            Some(result) => {
                let navigator = navigator.clone();
                let test_result = result.clone();
                let on_navigate_test_result = Callback::from(move |_| {
                    let navigator = navigator.clone();
                    let encoded = test_result.to_base64();
                    navigator.push(&Route::Results { code: encoded });
                });

                html! {
                    <div>
                    <button onclick={on_navigate_test_result}>{"View Certificate"}</button>
                    <TestResults<T,D> test={props.test.clone()} />
                    </div>
                }
            }
            None => player_input::<T>(
                total_answers,
                correct_answers,
                total_answers - correct_answers,
                Some(on_generate),
            ),
        },
    };

    html! {
        <div class="congratulations-container">
            <h2>{ i18n.t("Congratulations!")}</h2>
            <SoundPlayer sound_url="/fanfare-3-rpg.ogg" />
            <p>{format!("You have completed the test with a score of {:.1}% ({}/{})", performance, correct_answers, total_answers)}</p>
            {message::<T>(performance)}
            {certificate_part}
        </div>
    }
}

fn player_input<T: TypeTrait>(
    total_questions: usize,
    correct_answers: usize,
    incorrect_answers: usize,
    on_generate: Option<Callback<TestResult>>,
) -> Html {
    let test_type = T::get_t();

    html! {
        <PlayerInput test_type={test_type}
            total_questions={total_questions}
            correct_answers={correct_answers}
            incorrect_answers={incorrect_answers}
           on_generate={on_generate} />
    }
}

fn message<T: TypeTrait>(performance: f64) -> Html {
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
