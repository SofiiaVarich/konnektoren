use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Category<T, D> {
    pub category: T,
    pub details: Vec<D>,
}
