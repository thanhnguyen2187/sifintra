use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use diesel::SqliteConnection;
use diesel::query_dsl::positional_order_dsl::PositionalOrderClause;
use serde_json::json;
use snafu::prelude::*;
use std::sync::{LockResult, MutexGuard, PoisonError};
use uuid::Uuid;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu(display("ID may not be less than 10, but it was {id}"))]
    InvalidId { id: u16 },

    #[snafu(whatever, display("{message}"))]
    Whatever {
        message: String,
        #[snafu(source(from(Box<dyn std::error::Error>, Some)))]
        source: Option<Box<dyn std::error::Error>>,
    },

    #[snafu(display("I/O error: {source}"))]
    Io { source: std::io::Error },

    #[snafu(display("Unable to acquire database lock"))]
    DatabaseLock,

    #[snafu(display("Database insert error: {message}"))]
    DatabaseInsertError { message: String },

    #[snafu(display("Parse date error: {source}"))]
    ParseError { source: chrono::ParseError },

    #[snafu(display("Unreachable code"))]
    Unreachable,

    #[snafu(display("Database error: {source}"))]
    Database { source: diesel::result::Error },
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"success": false, "message": format!("{}", self)})),
        )
            .into_response()
    }
}

impl From<std::io::Error> for Error {
    fn from(source: std::io::Error) -> Self {
        Error::Io { source }
    }
}

impl From<diesel::result::Error> for Error {
    fn from(source: diesel::result::Error) -> Self {
        Error::Database { source }
    }
}

impl From<chrono::ParseError> for Error {
    fn from(source: chrono::ParseError) -> Self {
        Error::ParseError { source }
    }
}
