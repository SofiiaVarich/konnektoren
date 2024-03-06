use crate::model::KonnektorType;
use rand::seq::SliceRandom;
use yew::prelude::*;
use yew_bootstrap::component::{Button, ButtonGroup};
#[derive(Properties, PartialEq)]
pub struct TypeSelectorProps {
    pub on_select: Callback<KonnektorType>,
}

pub struct TypeSelector {
    button_order: Vec<KonnektorType>,
}

pub enum Msg {
    ButtonClicked(KonnektorType),
}

impl Component for TypeSelector {
    type Message = Msg;
    type Properties = TypeSelectorProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            button_order: vec![
                KonnektorType::Subjunktionen,
                KonnektorType::Konjunktionen,
                KonnektorType::Konjunktionaladverbien,
                KonnektorType::Infinitivgruppe,
                KonnektorType::BesonderePosition,
            ],
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ButtonClicked(konnektor_type) => {
                let mut rng = rand::thread_rng();
                self.button_order.shuffle(&mut rng);
                ctx.props().on_select.emit(konnektor_type);
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div id="typeselector" class="d-flex justify-content-center">
            <ButtonGroup>
            {for self.button_order.iter().map(|konnektor_type| {
                let button_text = match konnektor_type {
                    KonnektorType::Subjunktionen => "Nebensatz (Verb am Ende)",
                    KonnektorType::Konjunktionen => "Hauptsatz (Position 0)",
                    KonnektorType::Konjunktionaladverbien => "Hauptsatz (Position 1)",
                    KonnektorType::Infinitivgruppe => "Infinitivgruppe",
                    KonnektorType::BesonderePosition => "Besondere Position",
                };
                let konnektor_type_clone = konnektor_type.clone();
                html! {
                    <Button onclick={ctx.link().callback(move |_| Msg::ButtonClicked(konnektor_type_clone.clone()))}>
                        {button_text}
                    </Button>
                }
            })}
        </ButtonGroup>
            </div>
        }
    }
}
