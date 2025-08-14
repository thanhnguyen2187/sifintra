mod app_state;
mod db;
mod err;
mod frontend;
mod handlers;
mod schema;

use crate::app_state::AppState;
use crate::db::establish_connection;
use crate::err::Result;
use crate::handlers::{handle_hook_sepay, handle_stats};
use axum::{Router, routing::get, routing::post};
use dotenvy::dotenv;
use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let conn = establish_connection()?;
    let shared_state = Arc::new(Mutex::new(AppState { conn }));
    let app = Router::new()
        .route("/api/v1/health", get(async || "alive!"))
        .route("/api/v1/hooks/sepay", post(handle_hook_sepay))
        .route("/api/v1/stats", get(handle_stats))
        .fallback(frontend::static_handler)
        .with_state(shared_state);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;

    println!("Listening on http://127.0.0.1:3000");
    axum::serve(listener, app).await?;
    Ok(())
}
