use axum::Json;
use serde_json::{Value, json};

pub async fn handle_hook_sepay() -> Json<Value> {
    Json(json!({ "success": true }))
}
