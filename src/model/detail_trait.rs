use serde::Serialize;

pub trait DetailTrait:
    PartialEq + Clone + std::fmt::Debug + std::default::Default + Serialize
{
    fn get_detail(&self) -> String;
    fn get_example(&self) -> String;
}
