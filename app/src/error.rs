use axum::{response::{Response, IntoResponse}, http::StatusCode, Json};
use serde::Serialize;

#[derive(Serialize)]
pub struct ExternalExecutionError{
    pub step: String,
    pub message: String
}

impl IntoResponse for ExternalExecutionError {
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR, Json(&self)).into_response()
    }
}
