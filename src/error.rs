use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    LoginFail,
    TicketDeleteFailIdNotFound { id: i64 },
    AuthFailNoAuthTokenCookie,
    ServerError(String),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        match self {
            Error::LoginFail => (StatusCode::UNAUTHORIZED, "Login failed").into_response(),
            // _ => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error").into_response(),
            Error::TicketDeleteFailIdNotFound { id } => (
                StatusCode::NOT_FOUND,
                format!("Ticket delete failed, id not found: {}", id),
            )
                .into_response(),
            Error::AuthFailNoAuthTokenCookie => (
                StatusCode::UNAUTHORIZED,
                "Authentication failed, no auth token cookie",
            )
                .into_response(),
            Error::ServerError(msg) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                msg,
            )
                .into_response(),
        }
    }
}

impl From<String> for Error {
    fn from(msg: String) -> Self {
        Error::ServerError(msg)
    }
}

impl From<&str> for Error {
    fn from(msg: &str) -> Self {
        Error::ServerError(msg.to_string())
    }
}
