use serde::{Deserialize, Serialize};

// List
#[derive(Debug, Deserialize, Serialize)]
pub struct FilterOptions {
    pub page: Option<usize>,
    pub limit: Option<usize>,
}

// create
#[derive(Serialize, Deserialize, Debug)]
pub struct CreateNoteSchema {
    pub title: String,
    pub content: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_published: Option<bool>,
}

// update
#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateNoteSchema {
    pub title: Option<String>,
    pub content: Option<String>,
    pub is_published: Option<bool>,
}