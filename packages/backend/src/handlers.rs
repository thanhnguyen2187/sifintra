use crate::app_state::AppState;
use crate::db::{
    AmountType, RawSepay, UserTransaction, insert_raw_sepay, insert_user_transaction,
    sum_transaction_amount,
};
use crate::err::{Error, Result};
use axum::Json;
use axum::extract::{Query, State};
use chrono::format::Fixed::TimezoneOffset;
use chrono::{DateTime, NaiveDateTime};
use chrono::{FixedOffset, TimeZone};
use diesel::Connection;
use serde_derive::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

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
                transactionDate: payload.transaction_date.clone(),
                accountNumber: payload.account_number,
                subAccount: payload.sub_account.to_string(),
                code: payload.code.to_string(),
                content: payload.content,
                transferType: payload.transfer_type,
                description: payload.description.clone(),
                transferAmount: payload.transfer_amount as i32,
                referenceCode: payload.reference_code,
                accumulated: payload.accumulated as i32,
                id: payload.id as i32,
            },
        )?;
        if insert_count <= 0 {
            return Err(Error::DatabaseInsertError {
                message: format!(
                    "expected at least 1 record inserted to `raw__sepay`; got {}",
                    insert_count,
                )
                .to_string(),
            });
        }

        let date_timestamp_naive =
            NaiveDateTime::parse_from_str(&payload.transaction_date, "%Y-%m-%d %H:%M:%S")?;
        let offset = FixedOffset::east_opt(7 * 3_600).ok_or(Error::Unreachable)?;
        let date_timestamp = offset
            .from_local_datetime(&date_timestamp_naive)
            .single()
            .ok_or(Error::Unreachable)?;

        let insert_count = insert_user_transaction(
            &mut state.conn,
            &UserTransaction {
                id: Uuid::now_v7().to_string(),
                date_timestamp: date_timestamp.timestamp() as i32,
                description: payload.description,
                amount: if payload.transfer_amount > 0 {
                    payload.transfer_amount
                } else {
                    -payload.transfer_amount
                } as i32,
                category_id: None,
                source_id: "sepay".to_string(),
                created_at: None,
            },
        )?;
        if insert_count <= 0 {
            return Err(Error::DatabaseInsertError {
                message: format!(
                    "expected at least 1 record inserted to `user__transaction`; got {}",
                    insert_count,
                )
                .to_string(),
            });
        }

        return Ok(Json(json!({
            "success": true,
        })));
    }

    Err(Error::DatabaseLock)
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct StatsParams {
    from_timestamp: Option<i32>,
    to_timestamp: Option<i32>,
}

pub async fn handle_stats(
    Query(params): Query<StatsParams>,
    State(state_arc): State<Arc<Mutex<AppState>>>,
) -> Result<Json<Value>> {
    if let Ok(mut state) = state_arc.lock() {
        let income = sum_transaction_amount(
            &mut state.conn,
            &AmountType::Positive,
            params.from_timestamp,
            params.to_timestamp,
        )?;
        let expense = sum_transaction_amount(
            &mut state.conn,
            &AmountType::Negative,
            params.from_timestamp,
            params.to_timestamp,
        )?;
        let current = income - expense;
        return Ok(Json(json!({
            "totalIncomeVND": income,
            "totalExpenseVND": expense,
            "currentBalanceVND": current,
            "chartData": [],
        })));
    }

    Err(Error::DatabaseLock)
}
