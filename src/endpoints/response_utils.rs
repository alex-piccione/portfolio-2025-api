use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;

use crate::endpoints::models::common::{ErrorResponse, NewIdResponse};

pub fn _response_ok_no_data() -> Response {
    (StatusCode::OK).into_response()
}

pub fn response_ok<T: serde::Serialize>(data: T) -> Response {
    (StatusCode::OK, Json(data)).into_response()
}

pub fn response_created<T: serde::Serialize>(data: T) -> Response {
    (StatusCode::CREATED, Json(data)).into_response()
}

pub fn response_created_new_id(new_id: i32) -> Response {
    (StatusCode::CREATED, Json(NewIdResponse { new_id})).into_response()
}

// Errors
pub fn response_error(message: &str) -> Response {
    response_error_code(StatusCode::INTERNAL_SERVER_ERROR, message)
}

pub fn response_bad_request(message: &str) -> Response {
    response_error_code(StatusCode::BAD_REQUEST, message)
}

pub fn response_not_found(message: &str) -> Response {
    response_error_code(StatusCode::NOT_FOUND, message)
}

pub fn response_unhautorized(message: &str) -> Response {
    response_error_code(StatusCode::UNAUTHORIZED, message)
}

fn response_error_code(status_code: StatusCode, message: &str) -> Response {
    (status_code, Json(ErrorResponse::error(message)) ).into_response()
}