use crate::model::TestType;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct ExplanationProps {
    pub test_type: TestType,
}

#[function_component(Explanation)]
pub fn explanation(props: &ExplanationProps) -> Html {
    match props.test_type {
        TestType::Konnektoren => konnektoren_explanation(),
        TestType::Verbs => verbs_explanation(),
        _ => html! {<></>},
    }
}

fn konnektoren_explanation() -> Html {
    html! {
        <div class="explanation">
            <p>{"Konnektoren are conjunctions that connect two clauses or sentences. They are used to express the relationship between the two clauses or sentences."}</p>
            <p>{"There are three types of Konnektoren and two types of exceptions:"}</p>
            <ul>
                <li>{"Konnektoren mit Nebensatz (Verb am Ende) (= Subjunktionen)"}</li>
                <li>{"Konnektoren mit Hauptsatz (Position 0) (= Konjunktionen)"}</li>
                <li>{"Konnektoren mit Hauptsatz (Position 1) (= Konjunktionaladverbien)"}</li>
                <li>{"Konnektoren mit Infinitivgruppe"}</li>
                <li>{"Konnektoren mit besonderer Position"}</li>
            </ul>
            <p>{"Each type of Konnektor has its own set of rules and usage, and mastering them will help you express yourself more clearly and effectively in German."}</p>
            <hr />
            <h3>{"Your Task"}</h3>
            <p>{"You will be shown a Konnektor and you will have to select the correct group it belongs to. This exercise aims to help you understand the usage and position of different Konnektoren within a sentence."}</p>
            <p>{"To successfully complete this task, you should:"}</p>
            <ol>
                <li>{"First, find the Konnektor in the given sentence. Most sentences will have two parts, typically separated by a comma."}</li>
                <li>{"Identify to which part of the sentence the Konnektor belongs."}</li>
                <li>{"Next, find the verb in the sentence and analyze its position."}</li>
                <li>{"Based on the position of the verb and the part of the sentence the Konnektor belongs to, select the correct group of Konnektoren."}</li>
                <li>{"Words in the parentheses, behind the group name, are meant to help you memorize the rules of the Konnektor and the group to which it belongs."}</li>
            </ol>
            <p>{"By practicing the identification and correct classification of Konnektoren, you will enhance your ability to construct and understand complex German sentences."}</p>

            {konnektoren_explanation_test_mode()}
        </div>
    }
}

fn konnektoren_explanation_test_mode() -> Html {
    html! {
        <div class="explanation">
            <h2>{"Konnektoren: Test Your Knowledge"}</h2>
            <p>{"The Konnektoren test mode is designed for learners who want to challenge their understanding of German conjunctions without the need for examples."}</p>
            <p>{"Who needs it?"}</p>
            <ul>
                <li>{"Individuals looking to assess their knowledge of Konnektoren."}</li>
                <li>{"Those preparing for a German language exam and wishing to test themselves in advance."}</li>
                <li>{"Learners seeking a quick and efficient way to master all Konnektoren."}</li>
            </ul>
            <p>{"Why do you need it?"}</p>
            <ul>
                <li>{"Konnektoren are crucial for passing German exams."}</li>
                <li>{"Enhances understanding of the German language."}</li>
                <li>{"Common errors in using Konnektoren can cause misunderstandings and mistakes."}</li>
                <li>{"It's a fundamental aspect of the language that every learner should master."}</li>
            </ul>
            <p>{"Mastering Konnektoren is key to success in any German language examination. By learning them thoroughly, you will be able to speak and write in German confidently, clearly, and without mistakes. Prepare to impress with your flawless grammar and articulate expression!"}</p>
            <p class="encouragement">{"Embrace the challenge, elevate your German, and speak with pride. Your journey to German fluency starts now!"}</p>
        </div>
    }
}

fn verbs_explanation() -> Html {
    html! {
        <div class="explanation">
            <p>{"Verbs are the most important part of any sentence. They are the action words that express what the subject is doing. In German, verbs are conjugated to match the subject of the sentence, and they can be used in different tenses and moods."}</p>
            <p>{"There are three types of verbs in German:"}</p>
            <ul>
                <li>{"Regular verbs"}</li>
                <li>{"Irregular verbs"}</li>
                <li>{"Modal verbs"}</li>
            </ul>
            <p>{"Each type of verb has its own set of rules and usage, and mastering them will help you express yourself more clearly and effectively in German."}</p>
            <hr />
            <p>{"By practicing the identification and correct classification of verbs, you will enhance your ability to construct and understand complex German sentences."}</p>

            {verbs_explanation_test_mode()}
        </div>
    }
}

fn verbs_explanation_test_mode() -> Html {
    html! {
        <div class="explanation">
            <h2>{"Verben: Test Your Knowledge"}</h2>
            <p>{"The Verben test mode is designed for learners who want to challenge their understanding of German verbs without the need for examples."}</p>
            <p>{"Who needs it?"}</p>
            <ul>
                <li>{"Individuals looking to assess their knowledge of German verbs."}</li>
                <li>{"Those preparing for a German language exam and wishing to test themselves in advance."}</li>
                <li>{"Learners seeking a quick and efficient way to master all German verbs."}</li>
            </ul>
            <p>{"Why do you need it?"}</p>
            <ul>
                <li>{"Verbs are crucial for passing German exams."}</li>
                <li>{"Enhances understanding of the German language."}</li>
                <li>{"Common errors in using verbs can cause misunderstandings and mistakes."}</li>
                <li>{"It's a fundamental aspect of the language that every learner should master."}</li>
            </ul>
            <p>{"Mastering verbs is key to success in any German language examination. By learning them thoroughly, you will be able to speak and write in German confidently, clearly, and without mistakes. Prepare to impress with your flawless grammar and articulate expression!"}</p>
            <p class="encouragement">{"Embrace the challenge, elevate your German, and speak with pride. Your journey to German fluency starts now!"}</p>
        </div>
    }
}
