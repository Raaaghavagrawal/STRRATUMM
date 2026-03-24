use crate::models::Data;
use crate::state::AppState;
use axum::{extract::State, Json};
use chrono::Utc;
use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Deserialize, ToSchema)]
pub struct CreateDataRequest {
    #[schema(example = "New Entry Name")]
    pub name: String,
}

#[utoipa::path(
    post,
    path = "/data",
    request_body = CreateDataRequest,
    responses(
        (status = 201, description = "Data created successfully", body = Data)
    )
)]
pub async fn create_data(
    State(state): State<AppState>,
    Json(payload): Json<CreateDataRequest>,
) -> Json<Data> {
    let mut state = state.lock().unwrap();
    let new_data = Data {
        id: Uuid::new_v4(),
        name: payload.name,
        created_at: Utc::now(),
    };
    state.data_store.push(new_data.clone());
    Json(new_data)
}

#[utoipa::path(
    get,
    path = "/data",
    responses(
        (status = 200, description = "List all data stored in memory", body = [Data])
    )
)]
pub async fn get_all_data(State(state): State<AppState>) -> Json<Vec<Data>> {
    let state = state.lock().unwrap();
    Json(state.data_store.clone())
}
