use crate::app_state::AppState;
use crate::db::{AmountType, Category, RawSepay, UserTransaction, delete_category, insert_category, insert_raw_sepay, insert_user_transaction, select_categories, select_transactions, sum_transaction_amount, update_category, count_transactions};
use crate::err::{Error, Result};
use axum::Json;
use axum::extract::{Query, State};
use chrono::format::Fixed::TimezoneOffset;
use chrono::{DateTime, NaiveDateTime, Utc};
use chrono::{FixedOffset, TimeZone};
use diesel::Connection;
use serde_derive::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tracing::info;
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
                transferType: payload.transfer_type.clone(),
                description: payload.description.clone(),
                transferAmount: payload.transfer_amount as i32,
                referenceCode: payload.reference_code,
                accumulated: payload.accumulated as i32,
                id: payload.id as i32,
            },
        )?;
        info!(
            "Received SePay hook ID {}; transfer type {}; amount {}",
            payload.id,
            payload.transfer_type.clone(),
            payload.transfer_amount
        );
        if insert_count <= 0 {
            return Err(Error::DatabaseDataError {
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
                id: Some(Uuid::now_v7().to_string()),
                date_timestamp: date_timestamp.timestamp() as i32,
                description: payload.description,
                amount: if payload.transfer_type == "in" {
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
            return Err(Error::DatabaseDataError {
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
        let transactions = select_transactions(
            &mut state.conn,
            params.from_timestamp,
            params.to_timestamp,
            None,
            None,
        )?;
        let categories = select_categories(&mut state.conn)?;
        let categories_map: HashMap<String, String> =
            HashMap::from_iter(categories.into_iter().map(|category| {
                (
                    category.id.unwrap_or(String::from("_uncategorized")),
                    category.name,
                )
            }));

        let mut income = 0;
        let mut expense = 0;
        let mut current = 0;
        let mut expense_grouped: HashMap<String, i32> = HashMap::new();
        let mut chart_data = Vec::new();

        for transaction in transactions {
            match transaction.amount > 0 {
                true => {
                    income += transaction.amount;
                }
                false => {
                    expense += -transaction.amount;
                    let value = expense_grouped
                        .entry(
                            transaction
                                .category_id
                                .unwrap_or(String::from("_uncategorized")),
                        )
                        .or_insert(0);
                    *value += -transaction.amount;
                }
            }
            current += transaction.amount;
        }

        for (key, value) in expense_grouped {
            let label = categories_map
                .get(&key)
                .unwrap_or(&String::from("_uncategorized"))
                .clone();
            chart_data.push(json!({
                "label": label,
                "value": value,
            }));
        }

        return Ok(Json(json!({
            "data": {
                "totalIncomeVND": income,
                "totalExpenseVND": expense,
                "currentBalanceVND": current,
                "chartData": chart_data,
            }
        })));
    }

    Err(Error::DatabaseLock)
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct TransactionListParams {
    from_timestamp: Option<i32>,
    to_timestamp: Option<i32>,
    page: Option<i32>,
    limit: Option<i32>,
}

pub async fn handle_transaction_list(
    Query(params): Query<TransactionListParams>,
    State(state_arc): State<Arc<Mutex<AppState>>>,
) -> Result<Json<Value>> {
    if let Ok(mut state) = state_arc.lock() {
        let limit = params.limit.unwrap_or(10);
        let page = params.page.unwrap_or(1);

        let offset = (page - 1) * limit;
        let transactions = select_transactions(
            &mut state.conn,
            params.from_timestamp,
            params.to_timestamp,
            Some(offset),
            Some(limit),
        )?;
        let total = count_transactions(
            &mut state.conn,
            params.from_timestamp,
            params.to_timestamp,
        )?;

        return Ok(Json(json!({
            "data": transactions,
            "total": total,
        })));
    }

    Err(Error::DatabaseLock)
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionCreatePayload {
    amount: i32,
    description: String,
    date_timestamp: i32,
    category_id: Option<String>,
}

pub async fn handle_transaction_create(
    State(state_arc): State<Arc<Mutex<AppState>>>,
    Json(payload): Json<TransactionCreatePayload>,
) -> Result<Json<Value>> {
    if let Ok(mut state) = state_arc.lock() {
        insert_user_transaction(
            &mut state.conn,
            &UserTransaction {
                id: None,
                amount: payload.amount,
                description: payload.description,
                date_timestamp: payload.date_timestamp,
                category_id: payload.category_id,
                created_at: None,
                source_id: String::from("user"),
            },
        )?;

        return Ok(Json(json!({
            "success": true,
        })));
    }

    Err(Error::DatabaseLock)
}

pub async fn handle_category_list(
    State(state_arc): State<Arc<Mutex<AppState>>>,
) -> Result<Json<Value>> {
    if let Ok(mut state) = state_arc.lock() {
        let records = select_categories(&mut state.conn)?;

        return Ok(Json(json!({
            "data": records,
        })));
    }

    Err(Error::DatabaseLock)
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CategoryCreatePayload {
    name: String,
}

pub async fn handle_category_create(
    State(state_arc): State<Arc<Mutex<AppState>>>,
    Json(payload): Json<CategoryCreatePayload>,
) -> Result<Json<Value>> {
    if let Ok(mut state) = state_arc.lock() {
        let insert_count = insert_category(
            &mut state.conn,
            &Category {
                id: Some(Uuid::now_v7().to_string()),
                name: payload.name,
                created_at: None,
                updated_at: None,
            },
        )?;

        if insert_count != 1 {
            return Err(Error::DatabaseDataError {
                message: format!(
                    "error happened inserting; expected 1 record change; got {}",
                    insert_count,
                ),
            });
        }

        return Ok(Json(json!({
            "success": true,
        })));
    }

    Err(Error::DatabaseLock)
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CategoryUpdatePayload {
    id: String,
    name: String,
}

pub async fn handle_category_update(
    State(state_arc): State<Arc<Mutex<AppState>>>,
    Json(payload): Json<CategoryUpdatePayload>,
) -> Result<Json<Value>> {
    if let Ok(mut state) = state_arc.lock() {
        let insert_count = update_category(
            &mut state.conn,
            &Category {
                id: Some(payload.id),
                name: payload.name,
                created_at: None,
                updated_at: Some(Utc::now().format("%Y-%m-%d %H:%M:%S").to_string()),
            },
        )?;

        if insert_count != 1 {
            return Err(Error::DatabaseDataError {
                message: format!(
                    "error happened updating; expected 1 record change; got {}",
                    insert_count,
                ),
            });
        }

        return Ok(Json(json!({
            "success": true,
        })));
    }

    Err(Error::DatabaseLock)
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CategoryDeletePayload {
    id: String,
}

pub async fn handle_category_delete(
    State(state_arc): State<Arc<Mutex<AppState>>>,
    Json(payload): Json<CategoryDeletePayload>,
) -> Result<Json<Value>> {
    if let Ok(mut state) = state_arc.lock() {
        let count = delete_category(&mut state.conn, payload.id)?;

        if count != 1 {
            return Err(Error::DatabaseDataError {
                message: format!(
                    "error happened deleting; expected 1 record change; got {}",
                    count,
                ),
            });
        }

        return Ok(Json(json!({
            "success": true,
        })));
    }

    Err(Error::DatabaseLock)
}
