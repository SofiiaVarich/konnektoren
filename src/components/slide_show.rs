use gloo_storage::{LocalStorage, Storage as _};
use yew::prelude::*;

const SLIDE_SHOW_KEY: &str = "slide-show";

#[derive(Properties, PartialEq)]
pub struct SlideShowProps {
    pub urls: Vec<String>,
}

pub struct SlideShow {
    pub current_slide: usize,
}

impl SlideShow {
    fn save_current_slide(&self) {
        LocalStorage::set(SLIDE_SHOW_KEY, &self.current_slide.to_string()).unwrap();
    }

    fn load_current_slide(&mut self) {
        self.current_slide = LocalStorage::get(SLIDE_SHOW_KEY)
            .unwrap_or_else(|_| "0".to_string())
            .parse()
            .unwrap_or(0);
    }

    fn view_dot(&self, index: usize) -> Html {
        let class = if index < self.current_slide {
            "dot viewed"
        } else if index == self.current_slide {
            "dot active"
        } else {
            "dot"
        };
        html! { <span class={class}></span> }
    }
}

pub enum Msg {
    NextSlide,
    PreviousSlide,
    SkipSlideShow,
}

impl Component for SlideShow {
    type Message = Msg;
    type Properties = SlideShowProps;

    fn create(_ctx: &Context<Self>) -> Self {
        let mut slide_show = Self { current_slide: 0 };
        slide_show.load_current_slide();
        slide_show
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::NextSlide => {
                self.current_slide += 1;
                self.save_current_slide();
            }
            Msg::PreviousSlide => {
                self.current_slide -= 1;
                self.save_current_slide();
            }
            Msg::SkipSlideShow => {
                self.current_slide = usize::MAX;
                self.save_current_slide();
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        if self.current_slide >= ctx.props().urls.len() {
            return html! {};
        } else {
            html! {
                <div class="slide-show">
                    <img src={ctx.props().urls[self.current_slide].clone()} />
                    <div class="dots">
                        { for (0..ctx.props().urls.len()).map(|index| self.view_dot(index)) }
                    </div>
                    <button onclick={ctx.link().callback(|_| Msg::PreviousSlide)}>{ "Previous" }</button>
                    <button onclick={ctx.link().callback(|_| Msg::NextSlide)}>{ "Next" }</button>
                    <button onclick={ctx.link().callback(|_| Msg::SkipSlideShow)}>{ "Skip" }</button>
                </div>
            }
        }
    }
}
