use super::TypeSelector;
use crate::model::KonnektorTest;
use crate::model::KonnektorType;
use crate::model::Konnektoren;
use yew::prelude::*;
use yew_bootstrap::component::{card::*, Button};

#[derive(Properties, PartialEq)]
pub struct CarouselProps {
    pub konnektoren: Konnektoren,
}

pub struct KonnektorCarousel {
    test: KonnektorTest,
}

pub enum Msg {
    Next,
    Previous,
    SelectType(KonnektorType),
}

impl Component for KonnektorCarousel {
    type Message = Msg;
    type Properties = CarouselProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            test: KonnektorTest::new(&ctx.props().konnektoren),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Next => self.test.next(),
            Msg::Previous => self.test.prev(),
            Msg::SelectType(selected_type) => {}
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        if let Some(detail) = self.test.current() {
            html! {
                <div>
                    <Card>
                        <CardBody>
                            <CardTitle>{ &*detail.konnektor }</CardTitle>
                            <CardText>{ &*detail.example }</CardText>
                        </CardBody>
                    </Card>
                    <TypeSelector on_select={ctx.link().callback(Msg::SelectType)} />
                    <div class="d-flex justify-content-between mt-2">
                        <Button onclick={ctx.link().callback(|_| Msg::Previous)}>{ "Previous" }</Button>
                        <Button onclick={ctx.link().callback(|_| Msg::Next)}>{ "Next" }</Button>
                    </div>
                </div>
            }
        } else {
            html! { <p>{ "No Konnektoren found" }</p> }
        }
    }
}
