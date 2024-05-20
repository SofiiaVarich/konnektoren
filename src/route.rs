use yew_router::prelude::*;

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
    #[at("/config")]
    Config,
    #[at("/profile")]
    Profile,
    #[at("/history")]
    History,
    #[at("/leaderboard")]
    Leaderboard,
    #[at("/survey")]
    Survey,
    #[at("/results/:code")]
    Results { code: String },
}
