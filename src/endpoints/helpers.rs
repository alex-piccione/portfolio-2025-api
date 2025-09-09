use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;

pub fn response_error(status_code: StatusCode, message: &str) -> Response {
    (status_code, Json(format!("Error: {}", message))).into_response()
}

pub fn response_ok<T: serde::Serialize>(data: T) -> Response {
    (StatusCode::OK, Json(data)).into_response()
}

pub fn response_created<T: serde::Serialize>(data: T) -> Response {
    (StatusCode::CREATED, Json(data)).into_response()
}