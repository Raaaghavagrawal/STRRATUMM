use serde::Serialize;
use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};

#[derive(Serialize)]
pub struct AppError {
    pub message: String,
    pub status: u16,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (
            StatusCode::from_u16(self.status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),
            Json(self),
        ).into_response()
    }
}
