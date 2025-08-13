use crate::err::Result;
use diesel::prelude::*;
use snafu::ResultExt;
use std::env;
use serde_json::Value;

pub fn establish_connection() -> Result<SqliteConnection> {
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set either through environment variable or .env file");
    SqliteConnection::establish(&database_url)
        .with_whatever_context(|err| format!("Failed to connect to {}: {}", database_url, err))
}

pub fn insert_raw_sepay(conn: &mut SqliteConnection, raw_sepay: &RawSepay) -> Result<usize> {
    use crate::schema::raw__sepay::dsl::*;

    Ok(diesel::insert_into(raw__sepay).values(raw_sepay).execute(conn)?)
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::raw__sepay)]
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
