use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::NaiveDateTime;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Post {
    #[serde(skip_serializing_if = "Option::is_none")]  // Make id optional for input
    pub id: Option<Uuid>,
    pub title: String,
    pub body: String,
    #[serde(skip_serializing_if = "Option::is_none")]  // Make created_at optional for input
    pub created_at: Option<NaiveDateTime>,
}

// For creating posts
#[derive(Debug, Deserialize)]
pub struct CreatePost {
    pub title: String,
    pub body: String,
}