use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;

#[derive(Serialize)]
pub struct ExternalExecutionError {
    pub step: String,
    pub message: String,
    #[serde(skip_serializing)]
    pub status_code: StatusCode,
}

impl IntoResponse for ExternalExecutionError {
    fn into_response(self) -> Response {
        (self.status_code, Json(&self)).into_response()
    }
}
