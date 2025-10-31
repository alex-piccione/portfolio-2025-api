use axum::extract::State;
use axum::response::IntoResponse;
use axum::Extension;
use crate::endpoints::request_json_validator::ValidJson;
use crate::endpoints::request_validator::{RuleString};
use crate::endpoints::response_utils::*;
use crate::dependency_injection::AppState;
use crate::endpoints::models::custodian_models as models;
use crate::services::custodian_service::CreateError;
use crate::utils::auth_middleware::Session;
use crate::validate;

pub async fn create(
    State(state): State<AppState>, 
    Extension(session): Session,
    ValidJson(request): ValidJson<models::create::Request>) -> impl IntoResponse {

    match request.to_entity(session.user_id) {
        Ok(entity) => {

            validate!(
                "Name", &entity.name, RuleString::MinLength(3);
                "Custodian", &entity.custodian, RuleString::NotEmpty;
                //"Account", &entity.account, RuleStringOption::(2);                
            );

            match state.custodian_service.create(entity).await {
                Ok(new_id) => {
                    response_created_new_id(new_id)
                },
                Err(e) => match e {
                    CreateError::NameAlreadyExists => response_duplicated_value("Name"),
                    CreateError::Unexpected(message) => response_error(&message)
                } 
            }
        },
        Err(e) => response_bad_request(&e),
    }
}

pub async fn update(
    State(state): State<AppState>, 
    Extension(session): Session,
    ValidJson(request): ValidJson<models::update::Request>) -> impl IntoResponse {

    match request.to_entity(session.user_id) {
        Ok(entity) => {
            match state.custodian_service.update(entity).await {
                Ok(()) => response_ok("Custodian updated successfully"),
                Err(e) => response_error(e.as_str()),
            }
        },
        Err(e) => response_bad_request(&e),
    }
}

pub async fn list(State(state): State<AppState>) -> impl IntoResponse {
    match state.custodian_service.list().await {
        // no need to convert to a model
        Ok(entities) => response_ok(entities),
        Err(e) => response_error(e.as_str()),
    }
}
