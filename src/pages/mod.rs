use yew::prelude::*;
use yew_router::prelude::*;

mod about;
mod adjectives;
mod home;
mod konnektoren;
mod results;
mod verbs;

pub use about::About;
pub use adjectives::AdjectivesPage;
pub use home::Home;
pub use konnektoren::KonnektorenPage;
pub use results::Results;
pub use verbs::VerbsPage;

#[derive(Routable, PartialEq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/konnektoren")]
    Konnektoren,
    #[at("/adjectives")]
    Adjectives,
    #[at("/verbs")]
    Verbs,
    #[at("/about")]
    About,
    #[at("/results/:code")]
    Results { code: String },
}

#[function_component]
pub fn Navigation() -> Html {
    html! {
        <>
        <nav>
        <Link<Route> to={Route::Home}>{ "Home" }</Link<Route>>
        <Link<Route> to={Route::Konnektoren}>{ "Konnektoren" }</Link<Route>>
        <Link<Route> to={Route::Adjectives}>{ "Adjektive" }</Link<Route>>
        <Link<Route> to={Route::Verbs}>{ "Verben" }</Link<Route>>
        <Link<Route> to={Route::About}>{ "About" }</Link<Route>>
        </nav>
        </>
    }
}
