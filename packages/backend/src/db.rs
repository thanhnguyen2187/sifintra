use crate::err::Result;
use diesel::expression::BoxableExpression;
use diesel::prelude::*;
use serde_json::Value;
use snafu::ResultExt;
use std::env;
use serde_derive::Serialize;

pub fn establish_connection() -> Result<SqliteConnection> {
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set either through environment variable or .env file");
    SqliteConnection::establish(&database_url)
        .with_whatever_context(|err| format!("Failed to connect to {}: {}", database_url, err))
}

pub fn insert_raw_sepay(conn: &mut SqliteConnection, record: &RawSepay) -> Result<usize> {
    use crate::schema::raw__sepay::dsl::*;

    Ok(diesel::insert_into(raw__sepay)
        .values(record)
        .execute(conn)?)
}

pub fn insert_user_transaction(
    conn: &mut SqliteConnection,
    record: &UserTransaction,
) -> Result<usize> {
    use crate::schema::user__transaction::dsl::*;

    Ok(diesel::insert_into(user__transaction)
        .values(record)
        .execute(conn)?)
}

pub enum AmountType {
    Positive,
    Negative,
}

pub fn sum_transaction_amount(
    conn: &mut SqliteConnection,
    amount_type: &AmountType,
    from_timestamp: Option<i32>,
    to_timestamp: Option<i32>,
) -> Result<i32> {
    use crate::schema::user__transaction::dsl::*;

    let mut query = user__transaction.into_boxed();

    match amount_type {
        AmountType::Positive => query = query.filter(amount.gt(0)),
        AmountType::Negative => query = query.filter(amount.lt(0)),
    };

    if let Some(from_timestamp) = from_timestamp {
        query = query.filter(date_timestamp.ge(from_timestamp));
    }
    if let Some(to_timestamp) = to_timestamp {
        query = query.filter(date_timestamp.le(to_timestamp));
    }

    let total = query
        .select(diesel::dsl::sum(amount))
        .first::<Option<i64>>(conn)
        .map(|result| result.unwrap_or(0) as i32)?;

    Ok(total)
}

pub fn select_transactions(
    conn: &mut SqliteConnection,
    from_timestamp: Option<i32>,
    to_timestamp: Option<i32>,
    offset: Option<i32>,
    limit: Option<i32>,
) -> Result<Vec<UserTransaction>> {
    use crate::schema::user__transaction::dsl::*;

    let mut query = user__transaction.into_boxed();
    if let Some(from_timestamp) = from_timestamp {
        query = query.filter(date_timestamp.ge(from_timestamp));
    }
    if let Some(to_timestamp) = to_timestamp {
        query = query.filter(date_timestamp.le(to_timestamp));
    }
    if let Some(offset) = offset {
        query = query.offset(offset as i64);
    }
    if let Some(limit) = limit {
        query = query.limit(limit as i64);
    }

    let records = query.select(UserTransaction::as_select()).load(conn)?;

    Ok(records)
}

#[derive(Queryable, Insertable)]
#[diesel(table_name = crate::schema::raw__sepay)]
#[allow(non_snake_case)]
pub struct RawSepay {
    pub gateway: String,
    pub transactionDate: String,
    pub accountNumber: String,
    pub subAccount: String,
    pub code: String,
    pub content: String,
    pub transferType: String,
    pub description: String,
    pub transferAmount: i32,
    pub referenceCode: String,
    pub accumulated: i32,
    pub id: i32,
}

#[derive(Queryable, Selectable, Insertable, Serialize)]
#[diesel(table_name = crate::schema::user__transaction)]
#[diesel(check_for_backend(diesel::sqlite::Sqlite))]
pub struct UserTransaction {
    pub id: Option<String>,
    pub date_timestamp: i32,
    pub description: String,
    pub amount: i32,
    pub category_id: Option<String>,
    pub source_id: String,

    pub created_at: Option<String>,
}
