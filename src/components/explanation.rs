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
            // write your task based on the showed konnektor, to select the right group
            // generate: the user should 1. first find a shown konnektor in the sentence
            // all showed sentences have two parts. Mainly two sentences are seperated by the coma.
            // 2. the user should identify to which part of the sentance konnektor belongs to.
            // 3. nest step is to find a verb and analyse where is it based in the sentence.
            // 4. according to the position of the verb, the user should select the right group of the konnektor.
            // 5. words in the parenthesis, behind the gound name help the user to memorize the rules of the konnektor and group, where it belongs too.
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
        </div>
    }
}
