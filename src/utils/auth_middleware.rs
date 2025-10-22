use axum::{body::Body, extract::State, http::{Request}, middleware::Next, response::{IntoResponse}};
use crate::{endpoints::response_utils::{response_error, response_missing_auth_header, response_invalid_token}, services::auth_service::AuthError, utils::dependency_injection::AppState};

pub async fn requires_user(
    State(app_state):State<AppState>,
    mut req: Request<Body>, 
    next: Next) -> impl IntoResponse {

    crate::warn!("requires_user for {}", req.uri());

    let Some(access_token) = req
        .headers()
        .get("X-Auth-Token")
        .and_then(|v| v.to_str().ok())
        else {
            return response_missing_auth_header("X-Auth-Token HTTP header is missed in the request.");
        };

    // .and_then()  transform the inner value ONLY if it is Some
    // .to_str()  return Result<&str, ToStrError>
    // .ok()  converts Result to Option

    match app_state.auth_service.validate_access(access_token.to_string()).await {
        Ok(session) => {
            // Add User to the request
            req.extensions_mut().insert(session.user_id);

            next.run(req).await.into_response()
        }
        Err(AuthError::InvalidOrExpiredToken(info)) => response_invalid_token(format!("Access Token is invalid or expired. {}", info).as_str()),
        Err(AuthError::DatabaseError(e)) => {
            // log
            crate::error!("Something went wrong in the authentication process. {}", e);
            response_error("Something went wrong on authentication process")
        }
    }
}