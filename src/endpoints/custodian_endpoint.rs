use axum::extract::State;
use axum::response::IntoResponse;
use axum::Extension;
use crate::endpoints::request_json_validator::ValidJson;
use crate::endpoints::request_validator::{RuleString, RuleStringOption};
use crate::endpoints::response_utils::*;
use crate::dependency_injection::AppState;
use crate::endpoints::models::custodian_models as models;
use crate::utils::auth_middleware::Session;
use crate::validate;

pub async fn create(
    State(state): State<AppState>, 
    Extension(_session): Session,
    ValidJson(request): ValidJson<models::create::Request>) -> impl IntoResponse {
    match request.to_entity() {
        Ok(entity) => {

            let errors = validate!(
                "Name", &entity.name, RuleString::MinLength(3);
                "Account Country Code", &entity.account_country_code, RuleStringOption::FixLength(2);
            );

            if !errors.is_empty() {  // &errors.join(", ")
                return response_validation_errors(errors);
            }

            match state.custodian_service.create(entity).await {
                Ok(new_id) => {
                    let response = models::create::Response { new_id };
                    response_created(response)
                },
                Err(e) => response_error(e.as_str()),
            }
        },
        Err(e) => response_bad_request(&e),
    }
}

pub async fn update(State(state): State<AppState>, ValidJson(request): ValidJson<models::update::Request>) -> impl IntoResponse {
    match request.to_entity() {
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
