use super::TypeSelector;
use crate::components::TestProgressBar;
use crate::components::TestResults;
use crate::components::TestStatistics;
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
            Msg::SelectType(selected_type) => {
                self.test.answer_current(selected_type);
                self.test.next();
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        if let Some(detail) = self.test.current() {
            html! {
                <div>
                <TestProgressBar current={self.test.current_index() } total={self.test.len()} />
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
                    { self.test_results() }
                </div>
            }
        } else {
            html! { <p>{ "No Konnektoren found" }</p> }
        }
    }
}

impl KonnektorCarousel {
    fn test_results(&self) -> Html {
        if self.test.current_index() + 1 >= self.test.random_indices.len() {
            html! { <TestResults test={self.test.clone()} /> }
        } else {
            html! {
                <div>
                <TestStatistics test={self.test.clone()} />
                <TestResults test={self.test.clone()} />
                </div>
            }
        }
    }
}
