use crate::components::Logo;
use crate::route::Route;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component(MainMenu)]
pub fn main_menu() -> Html {
    let is_menu_open = use_state(|| false);

    let toggle_menu = {
        let is_menu_open = is_menu_open.clone();
        Callback::from(move |_| {
            is_menu_open.set(!*is_menu_open);
        })
    };

    let menu_class = if *is_menu_open {
        "menu-items active"
    } else {
        "menu-items"
    };

    html! {
        <div class="main-menu">
            <button class="burger-menu" aria-expanded={is_menu_open.to_string()} onclick={toggle_menu}>
                {"☰"}
            </button>

            if *is_menu_open {
                <div class={menu_class}>
                <br />
                    <Link<Route> to={Route::Home}>{html!{<Logo img_src={"/favicon.png".to_string()} />}}</Link<Route>>
                    <Link<Route> to={Route::Profile}>{ "Profile" }</Link<Route>>
                    <Link<Route> to={Route::History}>{ "History" }</Link<Route>>
                    <Link<Route> to={Route::Leaderboard}>{ "Leaderboard" }</Link<Route>>
                    <Link<Route> to={Route::About}>{ "About" }</Link<Route>>
                    <Link<Route> to={Route::Config}>{ "Config" }</Link<Route>>
                </div>
            }
        </div>
    }
}
