use yew::prelude::*;
use yew_router::prelude::*;

mod about;
mod home;
pub use about::About;
pub use home::Home;

#[derive(Routable, PartialEq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/about")]
    About,
}

#[function_component]
pub fn Navigation() -> Html {
    html! {
        <>
        <nav>
        <Link<Route> to={Route::Home}>{ "Home" }</Link<Route>>
        <Link<Route> to={Route::About}>{ "About" }</Link<Route>>
        </nav>
        </>
    }
}
