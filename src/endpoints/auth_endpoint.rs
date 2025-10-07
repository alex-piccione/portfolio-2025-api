use axum::{extract::State, response::IntoResponse, Json};
use crate::services::auth_service::LoginRequest;

use crate::services::user_service::CreateError;
use crate::{
    dependency_injection::AppState,
    endpoints::{helpers::*,
    models::auth::{signup, login}}
};

pub async fn signup(
    State(state): State<AppState>,
    Json(request): Json<signup::Request>
 ) -> impl IntoResponse {

    // TODO: validate request, use response_bad_request to return the 400 error with proper message

    let Some(currency) = state.currency_service.try_get(request.currency_id) else {
        return response_bad_request(&format!("Currency not found with ID={}", request.currency_id));
    };

    /* let currency = match state.currency_service.try_get(request.currency_id) {
        Some(c) => c,
        None => return response_bad_request(&format!("Currency not found with ID={}", request.currency_id))
    }; */

    match state.auth_service.signup(request.username, request.password, currency).await {
        Ok(_) => response_ok(signup::Response::success()),
        Err(CreateError::UsernameAlreadyInUse) => response_ok(signup::Response::error( "Username already taken")),
        Err(CreateError::DatabaseError(e)) => {
            // TODO: log error
            response_error(&e)
        }
    }
}

pub async fn login(
    State(state): State<AppState>, 
    Json(request): Json<login::Request>
) -> impl IntoResponse {

    // TODO: validate request

    let username = request.username.trim().to_string();
    let password = request.password.trim().to_string();

    let ip_address:String = String::from("");
    let user_agent:String = String::from("");

    let service_request = LoginRequest { username:username, password:password, ip_address:ip_address, user_agent:user_agent} ;

    match state.auth_service.login(service_request).await {
        Ok(session) =>  response_ok(
            login::Response::success(
                login::Session {
                        access_token: session.access_token,
                        access_token_expires_at: session.access_token_expires_at,
                        refresh_token: session.refresh_token,
                        refresh_token_expires_at: session.refresh_token_expires_at
            })),
        Err(e) => response_ok(login::Response::error(&e))
        //Err(e) => response_error(&e) 
    }
/* 
    match state.user_service.try_get_by_username(request.username).await {
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
    }    */
}