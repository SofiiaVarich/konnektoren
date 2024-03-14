use konnektoren::openapi_spec::OpenApiSpec;
use utoipa::OpenApi;

fn main() {
    let openapi_spec = OpenApiSpec::openapi().to_pretty_json().unwrap();
    std::fs::write("openapi.json", openapi_spec).unwrap();
}
