use axum::{body::Body, extract::State, http::{Request}, middleware::Next, response::{IntoResponse}};
use crate::{endpoints::response_utils::{response_bad_request, response_error, response_unhautorized}, services::auth_service::AuthError, utils::dependency_injection::AppState};

pub async fn requires_user(
    State(app_state):State<AppState>,
    req: Request<Body>, 
    next: Next) -> impl IntoResponse {
    let Some(access_token) = req
        .headers()
        .get("X-AUTH-TOKEN")
        .and_then(|v| v.to_str().ok())
        else {
            return response_bad_request("ACCESS TOKEN MISSED OR INVALID");
        };

    /*
    .and_then()  transorm the inner value ONLY if it is Some
    .to_str()  return Result<&str, ToStrError>
    .ok()  converts Result to Option
    */

    match app_state.auth_service.validate_access_token(access_token).await {
        Ok(_session_record) => {
            // You could add the user to request extensions if needed later
            // let mut req = req;
            // req.extensions_mut().insert(user);
            next.run(req).await.into_response()
        }
        Err(AuthError::ExpiredToken) => {
            // log
            response_unhautorized("ACCESS TOKEN EXPIRED")
        }
        Err(AuthError::InvalidToken) => {
            // log
            response_unhautorized("ACCESS TOKEN UNKNOWN")
        } 
        Err(AuthError::DatabaseError(e)) => {
            // log
            eprint!("Something went wrong on authentication process. {}", e);
            response_error("Something went wrong on authentication process")
        }
    }
}