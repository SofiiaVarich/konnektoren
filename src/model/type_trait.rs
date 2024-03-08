use serde::Serialize;

pub trait TypeTrait:
    PartialEq + Clone + std::default::Default + std::fmt::Display + std::fmt::Debug + Serialize
{
    fn get_type() -> String;
}
