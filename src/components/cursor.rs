use gloo_timers::callback::{Interval, Timeout};
use std::cell::RefCell;
use std::rc::Rc;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct CursorProps {
    pub length: usize,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Star {
    x: i32,
    y: i32,
    visible: bool,
}

impl Default for Star {
    fn default() -> Self {
        Star {
            x: 0,
            y: 0,
            visible: false,
        }
    }
}

impl Star {
    fn to_html(&self) -> Html {
        html! {
            <div class="glow-point star" style={format!("position: absolute; left: {}px; top: {}px; color: white; visibility: {}", self.x, self.y, if self.visible { "visible" } else { "hidden" })}>
                //{"⭐"}
            </div>
        }
    }
}

#[function_component(Cursor)]
pub fn cursor_component(props: &CursorProps) -> Html {
    let mouse_position = use_state(|| (0, 0));
    let stars = use_state(|| Rc::new(RefCell::new((0..props.length).map(|_| Star::default()).collect::<Vec<_>>())));
    let star_index = use_state(|| 0);
    let trigger_render = use_state(|| ());

    let on_mouse_move = {
        let mouse_position = mouse_position.clone();
        Callback::from(move |e: MouseEvent| {
            mouse_position.set((e.client_x(), e.client_y()));
        })
    };

    use_effect_with(mouse_position.clone(), {
        let stars = stars.clone();
        let length = props.length;
        let star_index = star_index.clone();
        move |mouse_position| {
            let (x, y) = **mouse_position;
            let index = *star_index % length; // Ensure index is always within bounds

            {
                let mut stars = stars.borrow_mut();
                if let Some(star) = stars.get_mut(index) {
                    star.x = x;
                    star.y = y;
                    star.visible = true;
                }
            }

            Timeout::new(1000, {
                let stars = stars.clone();
                move || {
                    let mut stars = stars.borrow_mut();
                    if let Some(star) = stars.get_mut(index) {
                        star.visible = false;
                    }
                    trigger_render.set(());
                }
            })
            .forget();

            star_index.set(index + 1);

            || ()
        }
    });

    html! {
        <div onmousemove={on_mouse_move} class="cursor-container">
            {
                for stars.borrow().iter().filter(|star| star.visible).map(|star| {
                    star.to_html()
                })
            }
        </div>
    }
}
