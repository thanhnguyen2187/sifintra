use crate::app_state::AppState;
use crate::db::{RawSepay, insert_raw_sepay};
use crate::err::{Error, Result};
use axum::Json;
use axum::extract::State;
use serde_derive::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::ops::{Deref, DerefMut};
use std::sync::{Arc, Mutex};

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

pub async fn handle_hook_sepay(
    State(state_arc): State<Arc<Mutex<AppState>>>,
    Json(payload): Json<Payload>,
) -> Result<Json<Value>> {
    if let Ok(mut state) = state_arc.lock() {
        let insert_count = insert_raw_sepay(
            &mut state.conn,
            &RawSepay {
                gateway: payload.gateway,
                transactionDate: payload.transaction_date,
                accountNumber: payload.account_number,
                subAccount: payload.sub_account.to_string(),
                code: payload.code.to_string(),
                content: payload.content,
                transferType: payload.transfer_type,
                description: payload.description,
                transferAmount: payload.transfer_amount as i32,
                referenceCode: payload.reference_code,
                accumulated: payload.accumulated as i32,
                id: payload.id as i32,
            },
        )?;
        return Ok(Json(json!({
            "success": true,
            "insert_count": insert_count,
        })));
    }

    Err(Error::DatabaseLock)
}
