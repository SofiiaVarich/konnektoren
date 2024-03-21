#[cfg(feature = "web")]
fn main() {
    yew::Renderer::<konnektoren::app::App>::new().render();
}
#[cfg(not(feature = "web"))]
fn main() {
    println!("This binary was built without the `web` feature")
}
