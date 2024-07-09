use axum::{response::IntoResponse, routing::get, Json, Router};
use dotenv::dotenv;
use handler::note_list_handler;
use sqlx::postgres::{PgPool, PgPoolOptions};

use std::sync::Arc;
use tokio::net::TcpListener;

mod handler;
mod models;
mod schema;

pub struct AppState {
    db: PgPool,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    println!("🌟 REST API Service 🌟");

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must set");
    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            println!("✅ Connection to the database is successful!");
            pool
        }
        Err(err) => {
            println!("❌ Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };

    let app = Router::new()
        .route("/ping", get(ping))
        .route("/notes", get(note_list_handler))
        .with_state(Arc::new(AppState { db: pool.clone() }));

    println!("Server startde successfully at 0.0.0.0:8080");

    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app.into_make_service())
        .await
        .unwrap();
}

pub async fn ping() -> impl IntoResponse {
    const MESSAGE: &str = "API Services";

    let json_response = serde_json::json!({
        "status": "ok",
        "message": MESSAGE
    });

    Json(json_response)
}
