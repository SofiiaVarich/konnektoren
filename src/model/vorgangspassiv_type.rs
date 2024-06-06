use super::{TestType, TypeTrait};
use serde::{Deserialize, Serialize};
use std::fmt;
use strum::EnumIter;
#[cfg(feature = "openapi")]
use utoipa::ToSchema;

#[derive(Default, Serialize, Deserialize, Debug, PartialEq, Clone, EnumIter)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
pub enum VorgangspassivType {
    #[default]
    #[serde(rename = "Präsens")]
    Praesens,
    #[serde(rename = "Präteritum")]
    Praeteritum,
    #[serde(rename = "Perfekt")]
    Perfekt,
    #[serde(rename = "Plusquamperfekt")]
    Plusquamperfekt,
    #[serde(rename = "Futur I")]
    Futur1,
    #[serde(rename = "Futur II")]
    Futur2,
}

impl fmt::Display for VorgangspassivType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            VorgangspassivType::Praesens => "Präsens",
            VorgangspassivType::Praeteritum => "Präteritum",
            VorgangspassivType::Perfekt => "Perfekt",
            VorgangspassivType::Plusquamperfekt => "Plusquamperfekt",
            VorgangspassivType::Futur1 => "Futur I",
            VorgangspassivType::Futur2 => "Futur II",
        };
        write!(f, "{}", s)
    }
}

impl TypeTrait for VorgangspassivType {
    fn get_type() -> String {
        "Einfaches Vorgangspassiv".to_string()
    }

    fn get_t() -> TestType {
        TestType::Vorgangspassiv
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct CategoryWrapper {
    category: VorgangspassivType,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_an() {
        let json = r#"{"category": "Präsens"}"#;
        let category: CategoryWrapper = serde_json::from_str(json).unwrap();
        assert_eq!(category.category, VorgangspassivType::Praesens);
    }
}
