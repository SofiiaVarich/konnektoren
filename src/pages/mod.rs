use yew::prelude::*;
use yew_router::prelude::*;

mod about;
mod home;
mod konnektoren;
mod results;
mod verbs;

pub use about::About;
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
        <Link<Route> to={Route::Verbs}>{ "Verbs" }</Link<Route>>
        <Link<Route> to={Route::About}>{ "About" }</Link<Route>>
        </nav>
        </>
    }
}
