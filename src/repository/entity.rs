use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct PipeLineEntity {
    pub id: Option<i64>,
    pub name: String,
    pub email: String,
    pub age: Option<u8>,
    pub created_at: DateTime<Utc>,
}

impl PipeLineEntity {
    pub(crate) fn new(id: Option<i64>, name: String, email: String, age: Option<u8>) -> Self {
        Self {
            id,
            name,
            email,
            age,
            created_at: Utc::now(),
        }
    }
}
