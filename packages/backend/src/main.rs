mod app_state;
mod db;
mod err;
mod frontend;
mod handlers;
mod schema;

use crate::app_state::AppState;
use crate::db::establish_connection;
use crate::err::Result;
use crate::handlers::{
    handle_category_create, handle_category_delete, handle_category_list, handle_category_update,
    handle_hook_sepay, handle_stats, handle_transaction_create, handle_transaction_delete,
    handle_transaction_list, handle_transaction_update,
};
use axum::routing::{delete, put};
use axum::{
    Router,
    body::Bytes,
    extract::MatchedPath,
    http::{HeaderMap, Request},
    response::{Html, Response},
    routing::{get, post},
};
use dotenvy::dotenv;
use std::sync::{Arc, Mutex};
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                // axum logs rejections from built-in extractors with the `axum::rejection`
                // target, at `TRACE` level. `axum::rejection=trace` enables showing those events
                format!(
                    "{}=debug,tower_http=debug,axum::rejection=trace",
                    env!("CARGO_CRATE_NAME")
                )
                .into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let conn = establish_connection()?;
    let shared_state = Arc::new(Mutex::new(AppState { conn }));
    let app = Router::new()
        .route("/api/v1/health", get(async || "alive!"))
        .route("/api/v1/hooks/sepay", post(handle_hook_sepay))
        .route("/api/v1/stats", get(handle_stats))
        .route("/api/v1/transactions", get(handle_transaction_list))
        .route("/api/v1/transactions", post(handle_transaction_create))
        .route("/api/v1/transactions", put(handle_transaction_update))
        .route("/api/v1/transactions", delete(handle_transaction_delete))
        .route("/api/v1/categories", get(handle_category_list))
        .route("/api/v1/categories", post(handle_category_create))
        .route("/api/v1/categories", put(handle_category_update))
        .route("/api/v1/categories", delete(handle_category_delete))
        .fallback(frontend::static_handler)
        .layer(
            CorsLayer::new()
                .allow_methods(Any)
                .allow_origin(Any)
                .allow_headers(Any),
        )
        .layer(TraceLayer::new_for_http())
        .with_state(shared_state);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;

    info!("Listening on http://127.0.0.1:3000");
    axum::serve(listener, app).await?;
    Ok(())
}
