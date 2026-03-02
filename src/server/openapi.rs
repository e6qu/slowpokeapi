use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::handlers::health::{HealthCheck, HealthResponse};

#[derive(OpenApi)]
#[openapi(
    info(
        title = "SlowPokeAPI",
        description = "Currency exchange rate API with distributed sync",
        version = "0.1.0",
        contact(
            name = "SlowPokeAPI",
            url = "https://github.com/e6qu/slowpokeapi"
        ),
        license(
            name = "MIT",
            url = "https://opensource.org/licenses/MIT"
        )
    ),
    tags(
        (name = "system", description = "System health and status endpoints")
    ),
    paths(
        crate::handlers::health::healthz,
        crate::handlers::health::readyz,
        crate::handlers::health::livez,
        crate::handlers::health::health
    ),
    components(
        schemas(HealthResponse, HealthCheck)
    )
)]
pub struct ApiDoc;

pub fn swagger_ui() -> SwaggerUi {
    SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi())
}
