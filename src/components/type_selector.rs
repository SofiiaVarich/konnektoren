use crate::model::{AdjectiveType, KonnektorType, TypeTrait, VerbType};
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

impl Component for TypeSelector<AdjectiveType> {
    type Message = Msg<AdjectiveType>;
    type Properties = TypeSelectorProps<AdjectiveType>;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            button_order: vec![
                AdjectiveType::An,
                AdjectiveType::Auf,
                AdjectiveType::Bei,
                AdjectiveType::Fuer,
                AdjectiveType::Mit,
                AdjectiveType::Ueber,
                AdjectiveType::Von,
                AdjectiveType::Zu,
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
                    AdjectiveType::An => "An",
                    AdjectiveType::Auf => "Auf",
                    AdjectiveType::Bei => "Bei",
                    AdjectiveType::Fuer => "Für",
                    AdjectiveType::Mit => "Mit",
                    AdjectiveType::Ueber => "Über",
                    AdjectiveType::Von => "Von",
                    AdjectiveType::Zu => "Zu",
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

impl Component for TypeSelector<VerbType> {
    type Message = Msg<VerbType>;
    type Properties = TypeSelectorProps<VerbType>;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            button_order: vec![
                VerbType::An,
                VerbType::Auf,
                VerbType::Fuer,
                VerbType::In,
                VerbType::Mit,
                VerbType::Nach,
                VerbType::Ueber,
                VerbType::Um,
                VerbType::Unter,
                VerbType::Von,
                VerbType::Vor,
                VerbType::Zu,
            ],
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ButtonClicked(verb_type) => {
                let mut rng = rand::thread_rng();
                self.button_order.shuffle(&mut rng);
                ctx.props().on_select.emit(verb_type);
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div id="typeselector" class="d-flex justify-content-center">
            <ButtonGroup>
            {for self.button_order.iter().map(|verb_type| {
                let button_text = match verb_type {
                    VerbType::An => "An",
                    VerbType::Auf => "Auf",
                    VerbType::Fuer => "Für",
                    VerbType::In => "In",
                    VerbType::Mit => "Mit",
                    VerbType::Nach => "Nach",
                    VerbType::Ueber => "Über",
                    VerbType::Um => "Um",
                    VerbType::Unter => "Unter",
                    VerbType::Von => "Von",
                    VerbType::Vor => "Vor",
                    VerbType::Zu => "Zu",
                };
                let verb_type_clone = verb_type.clone();
                html! {
                    <Button onclick={ctx.link().callback(move |_| Msg::ButtonClicked(verb_type_clone.clone()))}>
                        {button_text}
                    </Button>
                }
            })}
        </ButtonGroup>
            </div>
        }
    }
}
