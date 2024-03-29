#[cfg(feature = "web")]
pub mod app;
#[cfg(feature = "web")]
pub mod components;
pub mod model;
#[cfg(feature = "openapi")]
pub mod openapi_spec;

#[cfg(feature = "web")]
pub mod pages;
#[cfg(feature = "web")]
pub mod route;
pub mod utils;

#[cfg(feature = "tui")]
pub mod tui;
