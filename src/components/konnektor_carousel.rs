use super::TypeSelector;
use crate::components::Congratulations;
use crate::components::TestProgressBar;
use crate::components::TestResults;
use crate::components::TestStatistics;
use crate::model::AdjectiveDetail;
use crate::model::CategorizedItems;
use crate::model::CategorizedTest;
use crate::model::DetailTrait;
use crate::model::KonnektorDetail;
use crate::model::KonnektorType;
use crate::model::PrepositionType;
use crate::model::TypeTrait;
use yew::prelude::*;
use yew_bootstrap::component::{card::*, Button};

#[derive(Properties, PartialEq)]
pub struct CarouselProps<T: TypeTrait, D: DetailTrait> {
    pub konnektoren: CategorizedItems<T, D>,
}

pub struct KonnektorCarousel<T: TypeTrait, D: DetailTrait> {
    test: CategorizedTest<T, D>,
}

impl Default for KonnektorCarousel<KonnektorType, KonnektorDetail> {
    fn default() -> Self {
        Self {
            test: CategorizedTest::default(),
        }
    }
}

impl Default for KonnektorCarousel<PrepositionType, AdjectiveDetail> {
    fn default() -> Self {
        Self {
            test: CategorizedTest::default(),
        }
    }
}

pub enum Msg<T: TypeTrait> {
    Next,
    Previous,
    SelectType(T),
}

impl Component for KonnektorCarousel<KonnektorType, KonnektorDetail> {
    type Message = Msg<KonnektorType>;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self::default()
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
        if self.test.current_index() + 1 >= self.test.random_indices.len() {
            html! {
                    <div>
                        <Congratulations<KonnektorType, KonnektorDetail> test={self.test.clone()} />
                        <div class="d-flex justify-content-between mt-2">
                            <Button onclick={ctx.link().callback(|_| Msg::Previous)}>{ "Previous" }</Button>
                        </div>
                        { self.test_results() }
                    </div>
            }
        } else if let Some(detail) = self.test.current() {
            html! {
                <div>
                <TestProgressBar current={self.test.current_index() } total={self.test.len()} />
                    <Card>
                        <CardBody>
                            <CardTitle>{ &*detail.konnektor }</CardTitle>
                            <CardText>{ &*detail.example }</CardText>
                        </CardBody>
                    </Card>
                    <TypeSelector on_select={ctx.link().callback(Msg::SelectType::<KonnektorType>)} />
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

impl<T: TypeTrait + 'static, D: DetailTrait + 'static> KonnektorCarousel<T, D> {
    fn test_results(&self) -> Html {
        if self.test.current_index() + 1 >= self.test.random_indices.len() {
            html! { <TestResults<T, D> test={self.test.clone()} /> }
        } else {
            html! {
                <div>
                <TestStatistics<T, D> test={self.test.clone()} />
                <TestResults<T, D> test={self.test.clone()} />
                </div>
            }
        }
    }
}
