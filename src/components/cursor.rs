use gloo_timers::callback::Timeout;
use rand::Rng;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::closure::Closure;
use wasm_bindgen::JsCast;
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
        let mut rng = rand::thread_rng();
        let animation_class = format!("star glow-point fall-{}", rng.gen_range(1..=3));
        html! {

            <div class={animation_class} style={format!("position: absolute; left: {}px; top: {}px; color: white; visibility: {}", self.x, self.y, if self.visible { "visible" } else { "hidden" })}>
                //{"✨"}
            </div>
        }
    }
}

#[function_component(Cursor)]
pub fn cursor_component(props: &CursorProps) -> Html {
    let mouse_position = use_state(|| (0, 0));
    let stars = use_state(|| {
        Rc::new(RefCell::new(
            (0..props.length)
                .map(|_| Star::default())
                .collect::<Vec<_>>(),
        ))
    });
    let star_index = use_state(|| 0);
    let trigger_render = use_state(|| ());

    {
        let mouse_position = mouse_position.clone();
        use_effect_with((), move |_| {
            let closure = Closure::wrap(Box::new(move |event: MouseEvent| {
                mouse_position.set((event.client_x(), event.client_y()));
            }) as Box<dyn FnMut(_)>);

            web_sys::window()
                .and_then(|win| win.document())
                .expect("should have a document on window")
                .add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref())
                .expect("should register `mousemove` event");

            || {
                web_sys::window()
                    .and_then(|win| win.document())
                    .expect("should have a document on window")
                    .remove_event_listener_with_callback(
                        "mousemove",
                        closure.as_ref().unchecked_ref(),
                    )
                    .expect("should unregister `mousemove` event");
                closure.forget();
            }
        });
    }

    use_effect_with(mouse_position.clone(), {
        let stars = stars.clone();
        let length = props.length;
        let star_index = star_index.clone();
        move |mouse_position| {
            let (x, y) = **mouse_position;

            let scroll_x = web_sys::window().unwrap().scroll_x().unwrap() as i32;
            let scroll_y = web_sys::window().unwrap().scroll_y().unwrap() as i32;
            let x = x + scroll_x;
            let y = y + scroll_y;
            let index = *star_index % length;
            {
                let mut stars = stars.borrow_mut();
                if let Some(star) = stars.get_mut(index) {
                    star.x = x;
                    star.y = y;
                    star.visible = true;
                }
            }

            Timeout::new(1500, {
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
        <div class="cursor-container">
            {
                for stars.borrow().iter().filter(|star| star.visible).map(|star| {
                    star.to_html()
                })
            }
        </div>
    }
}
