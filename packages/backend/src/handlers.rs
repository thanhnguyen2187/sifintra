use axum::Json;
use serde_derive::{Deserialize, Serialize};
use serde_json::{Value, json};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Payload {
    pub gateway: String,
    pub transaction_date: String,
    pub account_number: String,
    pub sub_account: Value,
    pub code: Value,
    pub content: String,
    pub transfer_type: String,
    pub description: String,
    pub transfer_amount: i64,
    pub reference_code: String,
    pub accumulated: i64,
    pub id: i64,
}

pub async fn handle_hook_sepay(Json(payload): Json<Payload>) -> Json<Value> {
    Json(json!(payload))
}
