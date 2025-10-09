use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;

pub fn response_ok<T: serde::Serialize>(data: T) -> Response {
    (StatusCode::OK, Json(data)).into_response()
}

pub fn response_error(message: &str) -> Response {
    (StatusCode::INTERNAL_SERVER_ERROR, Json(format!("Error: {}", message))).into_response()
}

pub fn response_error_code(status_code: StatusCode, message: &str) -> Response {
    (status_code, Json(format!("Error: {}", message))).into_response()
}

pub fn response_bad_request(message: &str) -> Response {
    (StatusCode::BAD_REQUEST, Json(format!("Error: {}", message))).into_response()
}

pub fn response_not_found(message: &str) -> Response {
    (StatusCode::NOT_FOUND, Json( message)).into_response()
}

pub fn response_created<T: serde::Serialize>(data: T) -> Response {
    (StatusCode::CREATED, Json(data)).into_response()
}

pub fn response_unhautorized(message: &str) -> Response {
    response_error_code(StatusCode::UNAUTHORIZED, message)
}
