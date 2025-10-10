use axum::{extract::State, Json};
use axum::response::IntoResponse;
use crate::endpoints::response_utils::*;
use crate::dependency_injection::AppState;
use crate::endpoints::models::custodian_models as models;

pub async fn create(State(state): State<AppState>, Json(reqeust): Json<models::create::Request>) -> impl IntoResponse {
    match reqeust.to_entity() {
        Ok(entity) => {
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

pub async fn update(State(state): State<AppState>, Json(request): Json<models::update::Request>) -> impl IntoResponse {
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
