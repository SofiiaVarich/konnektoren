use serde::Serialize;
use strum::IntoEnumIterator;

use super::TestType;

pub trait TypeTrait:
    PartialEq
    + Clone
    + std::default::Default
    + std::fmt::Display
    + std::fmt::Debug
    + Serialize
    + IntoEnumIterator
{
    fn get_type() -> String;
    fn get_t() -> TestType;
}
