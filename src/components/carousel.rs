use super::TypeSelector;
use crate::components::CarouselCard;
use crate::components::Congratulations;
use crate::components::TestProgressBar;
use crate::components::TestResults;
use crate::components::TestStatistics;
use crate::model::CategorizedItems;
use crate::model::CategorizedTest;
use crate::model::DetailTrait;
use crate::model::TypeTrait;
use gloo_storage::{LocalStorage, Storage as _};
use serde::Deserialize;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct CarouselProps<T: TypeTrait + 'static, D: DetailTrait> {
    pub konnektoren: CategorizedItems<T, D>,
}

pub struct Carousel<T: TypeTrait, D: DetailTrait> {
    test: CategorizedTest<T, D>,
    hide_example: bool,
}

impl<T: TypeTrait, D: DetailTrait> Default for Carousel<T, D>
where
    CategorizedItems<T, D>: std::default::Default,
{
    fn default() -> Self {
        Self {
            test: CategorizedTest::default(),
            hide_example: true,
        }
    }
}

pub enum Msg<T: TypeTrait> {
    Next,
    Previous,
    SelectType(T),
    ToggleExampleVisibility,
    Reset,
}

impl<T: TypeTrait + 'static, D: DetailTrait + 'static> Carousel<T, D>
where
    T: for<'de> Deserialize<'de>,
    D: for<'de> Deserialize<'de>,
{
    fn test_results(&self) -> Html {
        if !self.test.is_finished() {
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

    fn save_test(&self) {
        LocalStorage::set(format!("test:{}", T::get_type()), self.test.clone()).unwrap();
    }

    fn load_test(&mut self) {
        if let Ok(test) = LocalStorage::get(format!("test:{}", T::get_type())) {
            self.test = test;
        }
    }
}

impl<T: TypeTrait + 'static, D: DetailTrait + 'static> Component for Carousel<T, D>
where
    CategorizedItems<T, D>: std::default::Default,
    T: for<'de> Deserialize<'de>,
    D: for<'de> Deserialize<'de>,
{
    type Message = Msg<T>;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let mut carousel = Self::default();
        carousel.load_test();
        carousel
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Next => self.test.next(),
            Msg::Previous => self.test.prev(),
            Msg::SelectType(selected_type) => {
                if !self.test.is_current_answered() {
                    self.test.answer_current(selected_type);
                    self.save_test()
                }
                self.test.next();
            }
            Msg::ToggleExampleVisibility => {
                self.hide_example = !self.hide_example;
                if !self.hide_example {
                    self.test.example_showed = true;
                    self.save_test()
                }
            }
            Msg::Reset => {
                self.test = CategorizedTest::default();
                self.save_test();
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        if self.test.is_finished() {
            html! {
                <div>
                    <Congratulations<T, D> test={self.test.clone()} />
                    <div class="action-buttons">
                        <button onclick={ctx.link().callback(|_| Msg::Previous)}>{ "Previous" }</button>
                        <button onclick={ctx.link().callback(|_| Msg::Reset)}>{ "Restart Test" }</button>
                        <div />
                    </div>
                    { self.test_results() }
                </div>
            }
        } else if let Some(detail) = self.test.current() {
            html! {<>
                <div class="carousel-background-image"></div>
                <div class="carousel">
                <TestProgressBar current={self.test.current_index() } total={self.test.len()} />
                    <CarouselCard<D> detail={detail.clone()} hide_example={self.hide_example} />
                        <TypeSelector<T> on_select={ctx.link().callback(Msg::SelectType::<T>)} />
                    <div class="action-buttons">
                        <button onclick={ctx.link().callback(|_| Msg::Previous)}>{ "Previous" }</button>
                        <button onclick={ctx.link().callback(|_| Msg::ToggleExampleVisibility)}>{ if self.hide_example { "Show Example" } else { "Hide Example" } }</button>
                        <button onclick={ctx.link().callback(|_| Msg::Reset)}>{ "Restart Test" }</button>
                        <button onclick={ctx.link().callback(|_| Msg::Next)}>{ "Next" }</button>
                    </div>
                    { self.test_results() }
                </div>
                </>
            }
        } else {
            html! { <p>{ format!("No {} found", T::get_type()) }</p> }
        }
    }
}
