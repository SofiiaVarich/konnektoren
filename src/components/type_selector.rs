use crate::model::KonnektorType;
use yew::prelude::*;
use yew_bootstrap::component::{Button, ButtonGroup};

#[derive(Properties, PartialEq)]
pub struct TypeSelectorProps {
    pub on_select: Callback<KonnektorType>,
}

pub struct TypeSelector;

impl Component for TypeSelector {
    type Message = KonnektorType;
    type Properties = TypeSelectorProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        ctx.props().on_select.emit(msg);
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div id="typeselector" class="d-flex justify-content-center">
                <ButtonGroup>
                    <Button onclick={ctx.link().callback(|_| KonnektorType::Subjunktionen)}>{ "Nebensatz (Verb am Ende)" }</Button>
                    <Button onclick={ctx.link().callback(|_| KonnektorType::Konjunktionen)}>{ "Hauptsatz (Position 0)" }</Button>
                    <Button onclick={ctx.link().callback(|_| KonnektorType::Konjunktionaladverbien)}>{ "Hauptsatz (Position 1)" }</Button>
                    <Button onclick={ctx.link().callback(|_| KonnektorType::Infinitivgruppe)}>{ "Infinitivgruppe" }</Button>
                    <Button onclick={ctx.link().callback(|_| KonnektorType::BesonderePosition)}>{ "Besondere Position" }</Button>
                </ButtonGroup>
            </div>
        }
    }
}
