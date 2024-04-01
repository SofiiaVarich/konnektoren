#[cfg(feature = "web")]
fn main() {
    use log::Level;
    console_log::init_with_level(Level::Trace).expect("error initializing log");

    yew::Renderer::<konnektoren::app::App>::new().render();
}
#[cfg(not(feature = "web"))]
fn main() {
    println!("This binary was built without the `web` feature")
}
