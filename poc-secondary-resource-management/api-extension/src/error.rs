use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use tracing::error;

// Source: https://github.com/tokio-rs/axum/blob/170d7d4dcc8a1368e7bea68f517a7791aff89422/examples/anyhow-error-response/src/main.rs
pub struct ServerError(anyhow::Error);

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        error!("Error during request: {}", self.0.root_cause());
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Something went wrong: {}", self.0),
        )
            .into_response()
    }
}

impl<E> From<E> for ServerError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}

pub type Result<T, E = ServerError> = std::result::Result<T, E>;
