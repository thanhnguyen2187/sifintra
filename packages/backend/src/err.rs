use axum::http::{StatusCode};
use axum::response::{IntoResponse, Response};
use snafu::prelude::*;

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
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self),
        ).into_response()
    }
}

impl From<std::io::Error> for Error {
    fn from(source: std::io::Error) -> Self {
        Error::Io { source }
    }
}