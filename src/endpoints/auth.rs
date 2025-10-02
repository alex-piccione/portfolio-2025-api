use axum::{extract::State, response::IntoResponse, Json};
use crate::endpoints::password_hashing::hash_password;
use crate::entities::user::User;
use crate::logic::currency_provider::CurrencyProvider;

use crate::utils::datetime::UtcDateTime;
use crate::{
    AppState,
    endpoints::{helpers::*,
    models::auth::{OkErrorResponse, signup, login}}
};

pub async fn _signup(
    State(state): State<AppState>,
    Json(request): Json<signup::Request>
 ) -> impl IntoResponse {

    // OkErrorResponse
    let id = uuid::Uuid::new_v4().to_string();

    let hashed_password = hash_password(&request.password);

    let currency = CurrencyProvider::all().iter().find(|item| item.id == request.currency_id).unwrap().clone();

    let user:User = User {
        id: id,
        username: request.username,
        hashed_password: hashed_password,
        creation_date:UtcDateTime::now(),
        currency
    };

    match state.users_repository.create(user).await {
        Ok(_) => response_ok(OkErrorResponse { is_success: true, error: None}),
        Err(e) => response_error(&e)
    }  
}

pub async fn login(
    State(_state): State<AppState>, 
    Json(request): Json<login::Request>
) -> impl IntoResponse {

    let is_ok = request.username == "demo" && request.password == "demo";

    // TODO
    response_ok(login::Response {is_success: is_ok, error: None})
}

// repeated template in all endpoints calls
/* 

    match state.currency_repository.list().await {
        Ok(entities) => {

           // some work here

            response_ok(models)
        },
        Err(e) => response_error(StatusCode::INTERNAL_SERVER_ERROR, e.as_str())

*/