use crate::model::{KonnektorType, PrepositionType, TypeTrait};
use rand::seq::SliceRandom;
use yew::prelude::*;
use yew_bootstrap::component::{Button, ButtonGroup};
#[derive(Properties, PartialEq)]
pub struct TypeSelectorProps<T: TypeTrait> {
    pub on_select: Callback<T>,
}

pub struct TypeSelector<T: TypeTrait> {
    button_order: Vec<T>,
}

pub enum Msg<T: TypeTrait> {
    ButtonClicked(T),
}

impl Component for TypeSelector<KonnektorType> {
    type Message = Msg<KonnektorType>;
    type Properties = TypeSelectorProps<KonnektorType>;

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

impl Component for TypeSelector<PrepositionType> {
    type Message = Msg<PrepositionType>;
    type Properties = TypeSelectorProps<PrepositionType>;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            button_order: vec![
                PrepositionType::An,
                PrepositionType::Auf,
                PrepositionType::Bei,
                PrepositionType::Fuer,
                PrepositionType::Mit,
                PrepositionType::Ueber,
                PrepositionType::Von,
                PrepositionType::Zu,
            ],
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ButtonClicked(preposition_type) => {
                let mut rng = rand::thread_rng();
                self.button_order.shuffle(&mut rng);
                ctx.props().on_select.emit(preposition_type);
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div id="typeselector" class="d-flex justify-content-center">
            <ButtonGroup>
            {for self.button_order.iter().map(|preposition_type| {
                let button_text = match preposition_type {
                    PrepositionType::An => "An",
                    PrepositionType::Auf => "Auf",
                    PrepositionType::Bei => "Bei",
                    PrepositionType::Fuer => "Für",
                    PrepositionType::Mit => "Mit",
                    PrepositionType::Ueber => "Über",
                    PrepositionType::Von => "Von",
                    PrepositionType::Zu => "Zu",
                };
                let preposition_type_clone = preposition_type.clone();
                html! {
                    <Button onclick={ctx.link().callback(move |_| Msg::ButtonClicked(preposition_type_clone.clone()))}>
                        {button_text}
                    </Button>
                }
            })}
        </ButtonGroup>
            </div>
        }
    }
}
