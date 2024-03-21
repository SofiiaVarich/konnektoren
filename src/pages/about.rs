use crate::components::Footer;
use yew::prelude::*;

#[function_component(About)]
pub fn about() -> Html {
    html! {
        <div class="about-page">
            <h1>{ "About this Learning Platform" }</h1>
            <p>
                { "This platform is dedicated to helping individuals improve their understanding and use of German grammar. Specifically, you can learn about:" }
            </p>
            <ul>
                <li><strong>{ "Konnektoren" }</strong>{ ": Understand how to connect clauses and sentences to improve the flow of your German writing and speaking." }</li>
                <li><strong>{ "Adjektive" }</strong>{ ": Dive into the use of adjectives, including those with prepositions, to add detail and depth to your descriptions." }</li>
                <li><strong>{ "Verben" }</strong>{ ": Master the use of verbs, particularly those with prepositions, to express actions and states more effectively." }</li>
            </ul>
            <p>
                { "Through interactive tests and comprehensive examples, this platform aims to enhance your German grammar skills, making you more confident in your language abilities." }
            </p>

            <Footer />
        </div>
    }
}
