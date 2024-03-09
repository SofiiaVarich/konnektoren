use super::TypeSelector;
use crate::components::CarouselCard;
use crate::components::Congratulations;
use crate::components::TestProgressBar;
use crate::components::TestResults;
use crate::components::TestStatistics;
use crate::model::AdjectiveDetail;
use crate::model::AdjectiveType;
use crate::model::CategorizedItems;
use crate::model::CategorizedTest;
use crate::model::DetailTrait;
use crate::model::KonnektorDetail;
use crate::model::KonnektorType;
use crate::model::TypeTrait;
use crate::model::VerbDetail;
use crate::model::VerbType;
use yew::prelude::*;
use yew_bootstrap::component::Button;

#[derive(Properties, PartialEq)]
pub struct CarouselProps<T: TypeTrait, D: DetailTrait> {
    pub konnektoren: CategorizedItems<T, D>,
}

pub struct Carousel<T: TypeTrait, D: DetailTrait> {
    test: CategorizedTest<T, D>,
    hide_example: bool,
}

impl Default for Carousel<KonnektorType, KonnektorDetail> {
    fn default() -> Self {
        Self {
            test: CategorizedTest::default(),
            hide_example: false,
        }
    }
}

impl Default for Carousel<AdjectiveType, AdjectiveDetail> {
    fn default() -> Self {
        Self {
            test: CategorizedTest::default(),
            hide_example: false,
        }
    }
}

impl Default for Carousel<VerbType, VerbDetail> {
    fn default() -> Self {
        Self {
            test: CategorizedTest::default(),
            hide_example: false,
        }
    }
}

pub enum Msg<T: TypeTrait> {
    Next,
    Previous,
    SelectType(T),
    ToggleExampleVisibility,
}

impl<T: TypeTrait + 'static, D: DetailTrait + 'static> Carousel<T, D> {
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

impl Component for Carousel<KonnektorType, KonnektorDetail> {
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
            Msg::ToggleExampleVisibility => {
                self.hide_example = !self.hide_example;
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
                    <CarouselCard<KonnektorDetail> detail={detail.clone()} hide_example={self.hide_example} />
                    <TypeSelector<KonnektorType> on_select={ctx.link().callback(Msg::SelectType::<KonnektorType>)} />
                    <div class="d-flex justify-content-between mt-2">
                        <Button onclick={ctx.link().callback(|_| Msg::Previous)}>{ "Previous" }</Button>
                        <Button onclick={ctx.link().callback(|_| Msg::ToggleExampleVisibility)}>{ if self.hide_example { "Show Example" } else { "Hide Example" } }</Button>
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

impl Component for Carousel<AdjectiveType, AdjectiveDetail> {
    type Message = Msg<AdjectiveType>;
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
            Msg::ToggleExampleVisibility => {
                self.hide_example = !self.hide_example;
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        if self.test.current_index() + 1 >= self.test.random_indices.len() {
            html! {
                    <div>
                        <Congratulations<AdjectiveType, AdjectiveDetail> test={self.test.clone()} />
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
                <CarouselCard<AdjectiveDetail> detail={detail.clone()} hide_example={self.hide_example} />
                    <TypeSelector<AdjectiveType> on_select={ctx.link().callback(Msg::SelectType::<AdjectiveType>)} />
                    <div class="d-flex justify-content-between mt-2">
                        <Button onclick={ctx.link().callback(|_| Msg::Previous)}>{ "Previous" }</Button>
                        <Button onclick={ctx.link().callback(|_| Msg::ToggleExampleVisibility)}>{ if self.hide_example { "Show Example" } else { "Hide Example" } }</Button>
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

impl Component for Carousel<VerbType, VerbDetail> {
    type Message = Msg<VerbType>;
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
            Msg::ToggleExampleVisibility => {
                self.hide_example = !self.hide_example;
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        if self.test.current_index() + 1 >= self.test.random_indices.len() {
            html! {
                    <div>
                        <Congratulations<VerbType, VerbDetail> test={self.test.clone()} />
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
                <CarouselCard<VerbDetail> detail={detail.clone()} hide_example={self.hide_example} />
                    <TypeSelector<VerbType> on_select={ctx.link().callback(Msg::SelectType::<VerbType>)} />
                    <div class="d-flex justify-content-between mt-2">
                        <Button onclick={ctx.link().callback(|_| Msg::Previous)}>{ "Previous" }</Button>
                        <Button onclick={ctx.link().callback(|_| Msg::ToggleExampleVisibility)}>{ if self.hide_example { "Show Example" } else { "Hide Example" } }</Button>
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
