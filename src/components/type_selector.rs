use crate::model::TypeTrait;
use rand::seq::SliceRandom;
use yew::prelude::*;
use yew_bootstrap::component::{Button, ButtonGroup};

#[derive(Properties, PartialEq)]
pub struct TypeSelectorProps<T: TypeTrait> {
    pub on_select: Callback<T>,
}

pub struct TypeSelector<T: TypeTrait + 'static> {
    button_order: Vec<T>,
}

pub enum Msg<T: TypeTrait> {
    ButtonClicked(T),
}

impl<T: TypeTrait> Component for TypeSelector<T> {
    type Message = Msg<T>;
    type Properties = TypeSelectorProps<T>;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            button_order: T::iter().collect(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ButtonClicked(t_type) => {
                let mut rng = rand::thread_rng();
                self.button_order.shuffle(&mut rng);
                ctx.props().on_select.emit(t_type);
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div id="typeselector" class="d-flex justify-content-center">
            <ButtonGroup>
            {for self.button_order.iter().map(|t_type| {
                let button_text = t_type.to_string();
                let t_type_clone = t_type.clone();
                html! {
                    <Button onclick={ctx.link().callback(move |_| Msg::ButtonClicked(t_type_clone.clone()))}>
                        {button_text}
                    </Button>
                }
            })}
        </ButtonGroup>
            </div>
        }
    }
}
