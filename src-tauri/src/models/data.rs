use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use utoipa::ToSchema;


#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Data {
    #[schema(example = "550e8400-e29b-41d4-a716-446655440000")]
    pub id: Uuid,
    #[schema(example = "Test Entry")]
    pub name: String,
    pub created_at: DateTime<Utc>,
}

