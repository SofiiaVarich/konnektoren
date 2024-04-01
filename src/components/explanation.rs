use crate::model::TestType;
use crate::utils::translation::LANGUAGE_KEY;
use gloo_storage::{LocalStorage, Storage};
use yew::prelude::*;
use yew_i18n::use_translation;

#[derive(Properties, PartialEq)]
pub struct ExplanationProps {
    pub test_type: TestType,
}

#[function_component(Explanation)]
pub fn explanation(props: &ExplanationProps) -> Html {
    match props.test_type {
        TestType::Konnektoren => html! { <KonnektorenExplanation /> },
        TestType::Adjectives => html! { <AdjectivesExplanation /> },
        TestType::Verbs => html! { <VerbsExplanation /> },
        TestType::Nomen => html! { <NomenExplanation /> },
    }
}

#[function_component(KonnektorenExplanation)]
fn konnektoren_explanation() -> Html {
    let mut i18n = use_translation();

    let selected_language =
        use_state(|| LocalStorage::get(LANGUAGE_KEY).unwrap_or_else(|_| "en".to_string()));

    let _ = i18n.set_translation_language(&selected_language);

    html! {
        <div class="explanation">
            <p>{ i18n.t("Konnektoren are conjunctions that connect two clauses or sentences. They are used to express the relationship between the two clauses or sentences.")}</p>
            <p>{ i18n.t("There are three types of Konnektoren and two types of exceptions:")}</p>
            <ul>
                <li>{ i18n.t("Konnektoren mit Nebensatz (Verb am Ende) (= Subjunktionen)")}</li>
                <li>{ i18n.t("Konnektoren mit Hauptsatz (Position 0) (= Konjunktionen)")}</li>
                <li>{ i18n.t("Konnektoren mit Hauptsatz (Position 1) (= Konjunktionaladverbien)")}</li>
                <li>{ i18n.t("Konnektoren mit Infinitivgruppe")}</li>
                <li>{ i18n.t("Konnektoren mit besonderer Position")}</li>
            </ul>
            <p>{ i18n.t("Each type of Konnektor has its own set of rules and usage, and mastering them will help you express yourself more clearly and effectively in German.")}</p>
            <hr />
            <h3>{ i18n.t("Your Task")}</h3>
            <p>{ i18n.t("You will be shown a Konnektor and you will have to select the correct group it belongs to. This exercise aims to help you understand the usage and position of different Konnektoren within a sentence.")}</p>
            <p>{ i18n.t("To successfully complete this task, you should:")}</p>
            <ol>
                <li>{ i18n.t("First, find the Konnektor in the given sentence. Most sentences will have two parts, typically separated by a comma.")}</li>
                <li>{ i18n.t("Identify to which part of the sentence the Konnektor belongs.")}</li>
                <li>{ i18n.t("Next, find the verb in the sentence and analyze its position.")}</li>
                <li>{ i18n.t("Based on the position of the verb and the part of the sentence the Konnektor belongs to, select the correct group of Konnektoren.")}</li>
                <li>{ i18n.t("Words in the parentheses, behind the group name, are meant to help you memorize the rules of the Konnektor and the group to which it belongs.")}</li>
            </ol>
            <p>{ i18n.t("By practicing the identification and correct classification of Konnektoren, you will enhance your ability to construct and understand complex German sentences.")}</p>

            <KonnektorenExplanationTestMode />
        </div>
    }
}

#[function_component(KonnektorenExplanationTestMode)]
fn konnektoren_explanation_test_mode() -> Html {
    let mut i18n = use_translation();

    let selected_language =
        use_state(|| LocalStorage::get(LANGUAGE_KEY).unwrap_or_else(|_| "en".to_string()));

    let _ = i18n.set_translation_language(&selected_language);
    html! {
        <div class="explanation">
            <h2>{ i18n.t("Konnektoren: Test Your Knowledge")}</h2>
            <p>{ i18n.t("The Konnektoren test mode is designed for learners who want to challenge their understanding of German conjunctions without the need for examples.")}</p>
            <p>{ i18n.t("Who needs it?")}</p>
            <ul>
                <li>{ i18n.t("Individuals looking to assess their knowledge of Konnektoren.")}</li>
                <li>{ i18n.t("Those preparing for a German language exam and wishing to test themselves in advance.")}</li>
                <li>{ i18n.t("Learners seeking a quick and efficient way to master all Konnektoren.")}</li>
            </ul>
            <p>{ i18n.t("Why do you need it?")}</p>
            <ul>
                <li>{ i18n.t("Konnektoren are crucial for passing German exams.")}</li>
                <li>{ i18n.t("Enhances understanding of the German language.")}</li>
                <li>{ i18n.t("Common errors in using Konnektoren can cause misunderstandings and mistakes.")}</li>
                <li>{ i18n.t("It's a fundamental aspect of the language that every learner should master.")}</li>
            </ul>
            <p>{ i18n.t("Mastering Konnektoren is key to success in any German language examination. By learning them thoroughly, you will be able to speak and write in German confidently, clearly, and without mistakes. Prepare to impress with your flawless grammar and articulate expression!")}</p>
            <p class="encouragement">{ i18n.t("Embrace the challenge, elevate your German, and speak with pride. Your journey to German fluency starts now!")}</p>
        </div>
    }
}

#[function_component(AdjectivesExplanation)]
fn adjectives_explanation() -> Html {
    let mut i18n = use_translation();

    let selected_language =
        use_state(|| LocalStorage::get(LANGUAGE_KEY).unwrap_or_else(|_| "en".to_string()));

    let _ = i18n.set_translation_language(&selected_language);
    html! {
        <div class="explanation">
            <p>{ i18n.t("Adjectives are words that describe or modify another person or thing in the sentence. In German, adjectives can precede the noun they describe, which is known as the attributive position, or they can appear in the predicate, following a form of the verb to be (sein), which is known as the predicative position.")}</p>
            <p>{ i18n.t("One of the key features of German adjectives is declension. German adjectives change their endings based on the gender, case, and number of the nouns they describe. This declension can seem daunting at first, but it is crucial for the correct sentence structure and meaning.")}</p>
            <p>{ i18n.t("There are three types of adjectives in German based on their declension patterns:")}</p>
            <ul>
                <li>{ i18n.t("Weak declension")}</li>
                <li>{ i18n.t("Strong declension")}</li>
                <li>{ i18n.t("Mixed declension")}</li>
            </ul>
            <p>{ i18n.t("Understanding when to use which declension is essential for mastering German adjectives.")}</p>
            <hr />
            <h3>{ i18n.t("Your Task")}</h3>
            <p>{ i18n.t("You will be presented with sentences where you must identify the correct form of the adjective based on the context of the sentence. This exercise will help you understand the declension of adjectives and how they change to match the nouns they describe.")}</p>
            <p>{ i18n.t("To successfully complete this task, consider the following steps:")}</p>
            <ol>
                <li>{ i18n.t("Identify the noun that the adjective describes.")}</li>
                <li>{ i18n.t("Determine the gender, case, and number of the noun.")}</li>
                <li>{ i18n.t("Select the correct adjective form based on the declension pattern.")}</li>
            </ol>
            <p>{ i18n.t("Practicing with a variety of sentences will help you become more comfortable with adjective declension and usage in German.")}</p>

            <AdjectivesExplanationTestMode />
        </div>
    }
}

#[function_component(AdjectivesExplanationTestMode)]
fn adjectives_explanation_test_mode() -> Html {
    let mut i18n = use_translation();

    let selected_language =
        use_state(|| LocalStorage::get(LANGUAGE_KEY).unwrap_or_else(|_| "en".to_string()));

    let _ = i18n.set_translation_language(&selected_language);
    html! {
        <div class="explanation">
            <h2>{ i18n.t("Adjectives: Test Your Knowledge")}</h2>
            <p>{ i18n.t("Challenge your understanding of German adjectives with this test mode, designed for learners seeking to assess and improve their adjective usage.")}</p>
            <p>{ i18n.t("Why it's beneficial:")}</p>
            <ul>
                <li>{ i18n.t("Assess your knowledge of adjective declension and placement.")}</li>
                <li>{ i18n.t("Identify areas for improvement in adjective usage.")}</li>
                <li>{ i18n.t("Prepare for German language exams with focused practice.")}</li>
            </ul>
            <p>{ i18n.t("Mastering adjectives is essential for expressing detailed and accurate descriptions in German. Take this opportunity to strengthen your skills and build confidence in your German language abilities.")}</p>
            <p class="encouragement">{ i18n.t("Dive into the challenge and emerge with a deeper understanding of German adjectives. Your path to German fluency is enriched with every step forward!")}</p>
        </div>
    }
}

#[function_component(VerbsExplanation)]
fn verbs_explanation() -> Html {
    let mut i18n = use_translation();

    let selected_language =
        use_state(|| LocalStorage::get(LANGUAGE_KEY).unwrap_or_else(|_| "en".to_string()));

    let _ = i18n.set_translation_language(&selected_language);
    html! {
        <div class="explanation">
            <p>{ i18n.t("Verbs are the most important part of any sentence. They are the action words that express what the subject is doing. In German, verbs are conjugated to match the subject of the sentence, and they can be used in different tenses and moods.")}</p>
            <p>{ i18n.t("There are three types of verbs in German:")}</p>
            <ul>
                <li>{ i18n.t("Regular verbs")}</li>
                <li>{ i18n.t("Irregular verbs")}</li>
                <li>{ i18n.t("Modal verbs")}</li>
            </ul>
            <p>{ i18n.t("Each type of verb has its own set of rules and usage, and mastering them will help you express yourself more clearly and effectively in German.")}</p>
            <hr />
            <p>{ i18n.t("By practicing the identification and correct classification of verbs, you will enhance your ability to construct and understand complex German sentences.")}</p>

            <VerbsExplanationTestMode />
        </div>
    }
}

#[function_component(VerbsExplanationTestMode)]
fn verbs_explanation_test_mode() -> Html {
    let mut i18n = use_translation();

    let selected_language =
        use_state(|| LocalStorage::get(LANGUAGE_KEY).unwrap_or_else(|_| "en".to_string()));

    let _ = i18n.set_translation_language(&selected_language);
    html! {
        <div class="explanation">
            <h2>{ i18n.t("Verben: Test Your Knowledge")}</h2>
            <p>{ i18n.t("The Verben test mode is designed for learners who want to challenge their understanding of German verbs without the need for examples.")}</p>
            <p>{ i18n.t("Who needs it?")}</p>
            <ul>
                <li>{ i18n.t("Individuals looking to assess their knowledge of German verbs.")}</li>
                <li>{ i18n.t("Those preparing for a German language exam and wishing to test themselves in advance.")}</li>
                <li>{ i18n.t("Learners seeking a quick and efficient way to master all German verbs.")}</li>
            </ul>
            <p>{ i18n.t("Why do you need it?")}</p>
            <ul>
                <li>{ i18n.t("Verbs are crucial for passing German exams.")}</li>
                <li>{ i18n.t("Enhances understanding of the German language.")}</li>
                <li>{ i18n.t("Common errors in using verbs can cause misunderstandings and mistakes.")}</li>
                <li>{ i18n.t("It's a fundamental aspect of the language that every learner should master.")}</li>
            </ul>
            <p>{ i18n.t("Mastering verbs is key to success in any German language examination. By learning them thoroughly, you will be able to speak and write in German confidently, clearly, and without mistakes. Prepare to impress with your flawless grammar and articulate expression!")}</p>
            <p class="encouragement">{ i18n.t("Embrace the challenge, elevate your German, and speak with pride. Your journey to German fluency starts now!")}</p>
        </div>
    }
}

#[function_component(NomenExplanation)]
fn nomen_explanation() -> Html {
    html! {
        <div class="explanation"></div>
    }
}
