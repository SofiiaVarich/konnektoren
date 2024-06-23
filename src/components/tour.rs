use yew::prelude::*;
use yew_tou_rs::prelude::{Tour as YewTour, TourConfig};

#[function_component(Tour)]
pub fn tour() -> Html {
    let config: TourConfig = serde_yaml::from_str(include_str!("../assets/tour.yml")).unwrap();

    html! {
    <div>
    <YewTour steps={config.steps} />
    </div>
    }
}
