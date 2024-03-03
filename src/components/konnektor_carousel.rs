use super::konnektor_card::KonnektorCard;
use crate::model::{KonnektorCategory, KonnektorDetail, KonnektorType, Konnektoren};
use serde::{Deserialize, Serialize};
use yew::prelude::*;
use yew_bootstrap::component::{card::*, Button};

#[derive(Properties, PartialEq)]
pub struct CarouselProps {
    pub konnektoren: Vec<KonnektorDetail>, // Assuming you want to pass a flat list of all details
}

pub struct KonnektorCarousel {
    konnektoren: Vec<KonnektorDetail>,
    current_index: usize,
}

pub enum Msg {
    Next,
    Previous,
}

impl Component for KonnektorCarousel {
    type Message = Msg;
    type Properties = CarouselProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            konnektoren: ctx.props().konnektoren.clone(),
            current_index: 0,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Next => {
                if self.current_index < self.konnektoren.len() - 1 {
                    self.current_index += 1;
                }
            }
            Msg::Previous => {
                if self.current_index > 0 {
                    self.current_index -= 1;
                }
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let konnektoren = &ctx.props().konnektoren;
        if konnektoren.is_empty() {
            return html! { <p>{ "No Konnektoren found" }</p> };
        }

        let detail = &konnektoren[self.current_index];

        html! {
            <div>
                <Card>
                    <CardBody>
                        <CardTitle>{ &*detail.konnektor }</CardTitle>
                        <CardText>{ &*detail.example }</CardText>
                    </CardBody>
                </Card>
                <div class="d-flex justify-content-between mt-2">
                    <Button onclick={ctx.link().callback(|_| Msg::Previous)}>{ "Previous" }</Button>
                    <Button onclick={ctx.link().callback(|_| Msg::Next)}>{ "Next" }</Button>
                </div>
            </div>
        }
    }
}
