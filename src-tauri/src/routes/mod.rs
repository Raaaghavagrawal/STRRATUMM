use axum::{
    routing::{get, post},
    Router,
};
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::handlers::{
    data::{create_data, get_all_data, CreateDataRequest},
    health::health_check,
};
use crate::models::Data;
use crate::state::AppState;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::handlers::health::health_check,
        crate::handlers::data::create_data,
        crate::handlers::data::get_all_data,
    ),
    components(
        schemas(Data, CreateDataRequest)
    ),
    tags(
        (name = "Tauri Hybrid API", description = "Endpoints for the hybrid desktop + backend application")
    )
)]
pub struct ApiDoc;

pub fn create_router(state: AppState) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        .merge(SwaggerUi::new("/docs").url("/api-doc/openapi.json", ApiDoc::openapi()))
        .route("/health", get(health_check))
        .route("/data", post(create_data))
        .route("/data", get(get_all_data))
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}
