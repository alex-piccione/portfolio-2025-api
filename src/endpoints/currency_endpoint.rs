use axum::{extract::Path,  extract::State, response::IntoResponse};

use super::response_utils::{response_ok, response_error, response_created};
use crate::endpoints::request_json_validator::ValidJson;
use crate::endpoints::response_utils::{response_bad_request, response_not_found};
use crate::dependency_injection::AppState;
use crate::endpoints::models::currency_models as models;

pub async fn create(State(state): State<AppState>, ValidJson(data): ValidJson<models::CreateRequest>) -> impl IntoResponse {
    match data.to_entity() {
        Ok(entity) => {
            match state.currency_service.create(entity).await {
                Ok(new_id) => {
                    let response = models::CreateResponse { new_id };
                    response_created(response)
                },
                Err(e) => response_error(&e)
            }
        },
        Err(e) => response_bad_request(&e)
    }
}

pub async fn update(State(state): State<AppState>, ValidJson(data): ValidJson<models::UpdateRequest>) -> impl IntoResponse {
    match data.to_entity() {
        Ok(entity) => {
            match state.currency_service.update(entity).await {
                Ok(()) => response_ok("Currency updated successfully"),
                Err(e) => response_error(&e),
            }
        },
        Err(e) => response_bad_request(&e)
    }
}

pub async fn single(State(state):State<AppState>, Path(id):Path<i32>) -> impl IntoResponse {    
    match state.currency_service.try_get(id) {
        Some(currency) => response_ok(models::Currency::from(currency)),
        None => response_not_found(&format!("Currency not found with id = {}", id))
    } 
}
    
pub async fn list(State(state): State<AppState>) -> impl IntoResponse {
    let entities = state.currency_service.all();
    let models:Vec<models::Currency> = entities.iter().map(|e|models::Currency::from(e.clone())).collect();
    response_ok(models)
}
