use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, sqlx::FromRow)]
#[allow(non_snake_case)]
pub struct NoteModel {
    pub id: String,
    pub title: String,
    pub content: String,
    pub is_published: i8,
    /* pub created_at: Option<chrono::DateTime<chrono::Utc>>, */
    pub created_at: Option<chrono::NaiveDateTime>,
    /* pub updated_at: Option<chrono::DateTime<chrono::Utc>>, */
    pub updated_at: Option<chrono::NaiveDateTime>,
}

#[derive(Debug, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct NoteModelResponse {
    pub id: String,
    pub title: String,
    pub content: String,
    pub is_published: bool,
    // pub created_at: chrono::DateTime<chrono::Utc>,
    // pub updated_at: chrono::DateTime<chrono::Utc>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}
