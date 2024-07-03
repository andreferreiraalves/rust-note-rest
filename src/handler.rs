use std::sync::Arc;

use crate::{models::*, schema::*, AppState};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;

fn to_note_response(note: &NoteModel) -> NoteModelResponse {
    NoteModelResponse {
        id: note.id.to_owned(),
        title: note.title.to_owned(),
        content: note.content.to_owned(),
        is_published: note.is_published != 0,
        created_at: note.created_at.unwrap(),
        updated_at: note.updated_at.unwrap(),
    }
}

pub async fn note_list_handler(
    opts: Option<Query<FilterOptions>>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let Query(opts) = opts.unwrap();

    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    let notes = sqlx::query_as!(
        NoteModel,
        r#"SELECT * FROM notes ORDER by id limit $1 offset $2"#,
        limit as i32,
        offset as i32,
    )
    .fetch_all(&data.db)
    .await
    .map_err(|e| {
        let error_response = serde_json::json!({
            "status":"error",
            "message": format!("Database error: {}", e),
        });

        (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
    })?;

    let note_responses = notes
        .iter()
        .map(|note| to_note_response(&note))
        .collect::<Vec<NoteModelResponse>>();

    let json_response = serde_json::json!({
        "status": "ok",
        "count": note_responses.len(),
        "notes" : note_responses

    });

    Ok(Json(json_response))
}

pub async fn create_note_handler(
    State(data): State<Arc<AppState>>,
    Json(body): Json<CreateNoteSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let id = uuid::Uuid::new_v4().to_string();
    let query_result = sqlx::query(r#"INSERT INTO notes (id, title, content) values($1, $2, $3)"#)
        .bind(&id)
        .bind(&body.title)
        .bind(&body.content)
        .execute(&data.db)
        .await
        .map_err(|err: sqlx::Error| err.to_string());

    if let Err(err) = query_result {
        if err.contains("Duplicate entry") {
            let error_response = serde_json::json!({
                "status" : "error",
                "message": "Note already exists",
            });

            return Err((StatusCode::CONFLICT, Json(error_response)));
        }

        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
            "status": "error", "message": format!("{:?}",err )       })),
        ));
    }

    let note = sqlx::query_as!(NoteModel, r#"select * from notes where id = $1"#, &id)
        .fetch_one(&data.db)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!(
                    {
                        "status": "error",
                        "message":format!("{:?}",e),
                    }
                )),
            )
        })?;

    let note_response = serde_json::json!({
        "status": "success",
        "data": serde_json::json!(&note)

    });

    Ok(Json(note_response))
}
