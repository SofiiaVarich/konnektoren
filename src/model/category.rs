use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, ToSchema)]
pub struct Category<T, D> {
    pub category: T,
    pub details: Vec<D>,
}
