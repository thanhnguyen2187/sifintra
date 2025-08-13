mod err;
mod frontend;
mod handlers;

use crate::handlers::handle_hook_sepay;
use axum::{Router, routing::get, routing::post};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/api/v1/health", get(async || "alive!"))
        .route("/api/v1/hooks/sepay", post(handle_hook_sepay))
        .fallback(frontend::static_handler);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    println!("Listening on http://127.0.0.1:3000");
    axum::serve(listener, app).await.unwrap();
}
