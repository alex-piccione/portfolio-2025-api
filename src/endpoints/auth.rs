use axum::{extract::State, response::IntoResponse, Json};

use crate::{
    endpoints::{helpers::response_ok, models::auth::login}, AppState
};

pub async fn login(
    State(_state): State<AppState>, 
    Json(request): Json<login::Request>
) -> impl IntoResponse {

    let is_ok = request.username == "demo" && request.password == "demo";

    // TODO
    return response_ok(login::Response {is_success: is_ok, error: None});
}