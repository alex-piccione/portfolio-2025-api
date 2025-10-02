use axum::{extract::State, response::IntoResponse, Json};
use crate::endpoints::password_hashing::{hash_password, verify_password};
use crate::entities::user::User;
use crate::logic::currency_provider::CurrencyProvider;

use crate::utils::datetime::UtcDateTime;
use crate::{
    AppState,
    endpoints::{helpers::*,
    models::auth::{OkErrorResponse, signup, login}}
};

pub async fn signup(
    State(state): State<AppState>,
    Json(request): Json<signup::Request>
 ) -> impl IntoResponse {

    let id = uuid::Uuid::new_v4().to_string();

    let hashed_password = hash_password(&request.password);

    let currency = CurrencyProvider::all().iter().find(|item| item.id == request.currency_id).unwrap().clone();

    let user:User = User {
        id: id,
        username: request.username,
        hashed_password: hashed_password,
        creation_date:UtcDateTime::now(),
        currency,
        role: String::from("User"), // default
    };

    match state.user_repository.create(user).await {
        Ok(_) => response_ok(OkErrorResponse { is_success: true, error: None}),
        Err(e) => response_error(&e)
    }  
}

pub async fn login(
    State(state): State<AppState>, 
    Json(request): Json<login::Request>
) -> impl IntoResponse {

    match state.user_repository.get_by_username(request.username).await {
        Ok(option) => {
            match option {
                Some(record) => {
                    match verify_password(&request.password, &record.hashed_password) {
                        true => response_ok(OkErrorResponse {is_success: true, error: None}),
                        false => response_ok(OkErrorResponse {is_success: false, error: None}),
                    }                    
                },
                None => response_ok(OkErrorResponse {is_success: false, error: None}),
            }
        },
        Err(e) => response_error(e.as_str())
    }    
}