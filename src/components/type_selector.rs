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
            <ButtonGroup>
                <Button onclick={ctx.link().callback(|_| KonnektorType::Subjunktionen)}>{ "Subjunktionen" }</Button>
                <Button onclick={ctx.link().callback(|_| KonnektorType::Konjunktionen)}>{ "Konjunktionen" }</Button>
                <Button onclick={ctx.link().callback(|_| KonnektorType::Konjunktionaladverbien)}>{ "Konjunktionaladverbien" }</Button>
                <Button onclick={ctx.link().callback(|_| KonnektorType::Infinitivgruppe)}>{ "Infinitivgruppe" }</Button>
                <Button onclick={ctx.link().callback(|_| KonnektorType::BesonderePosition)}>{ "Besondere Position" }</Button>
            </ButtonGroup>
        }
    }
}
