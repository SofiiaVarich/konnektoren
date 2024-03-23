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
use yew::prelude::*;
use yew_bootstrap::component::Button;

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
        if self.test.is_finished() {
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

impl<T: TypeTrait + 'static, D: DetailTrait + 'static> Component for Carousel<T, D>
where
    CategorizedItems<T, D>: std::default::Default,
{
    type Message = Msg<T>;
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
        if self.test.is_finished() {
            html! {
                    <div>
                        <Congratulations<T, D> test={self.test.clone()} />
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
                    <CarouselCard<D> detail={detail.clone()} hide_example={self.hide_example} />
                        <TypeSelector<T> on_select={ctx.link().callback(Msg::SelectType::<T>)} />
                    <div class="d-flex justify-content-between mt-2">
                        <Button onclick={ctx.link().callback(|_| Msg::Previous)}>{ "Previous" }</Button>
                        <Button onclick={ctx.link().callback(|_| Msg::ToggleExampleVisibility)}>{ if self.hide_example { "Show Example" } else { "Hide Example" } }</Button>
                        <Button onclick={ctx.link().callback(|_| Msg::Next)}>{ "Next" }</Button>
                    </div>
                    { self.test_results() }
                </div>
            }
        } else {
            html! { <p>{ format!("No {} found", T::get_type()) }</p> }
        }
    }
}
