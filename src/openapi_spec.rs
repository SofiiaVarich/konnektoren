use crate::model::*;
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(components(schemas(
    AdjectiveDetail,
    AdjectiveType,
    Adjectives,
    KonnektorDetail,
    KonnektorType,
    VerbDetail,
    VerbType
)))]
pub struct OpenApiSpec;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_openapi_spec() {
        let openapi_spec = OpenApiSpec::openapi().to_pretty_json().unwrap();
        assert!(openapi_spec.contains(r#""AdjectiveDetail""#));
    }
}
