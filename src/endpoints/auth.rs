use axum::{extract::State, response::IntoResponse, Json};

use crate::{
    endpoints::{helpers::response_ok, models::auth::login}, AppState
};

pub async fn login(
    State(state): State<AppState>, 
    Json(data): Json<login::Request>
) -> impl IntoResponse {

    // TODO
    return response_ok(login::Response {is_success: true, error: None});
}