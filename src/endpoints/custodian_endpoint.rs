use axum::{extract::State, Json};
use axum::response::IntoResponse;
use crate::endpoints::helpers::*;
use crate::dependency_injection::AppState;
use crate::endpoints::models::custodian as models;

pub async fn create(State(state): State<AppState>, Json(data): Json<models::CreateRequest>) -> impl IntoResponse {
    match data.to_entity() {
        Ok(entity) => {
            match state.custodian_service.create(entity).await {
                Ok(new_id) => {
                    let response = models::CreateResponse { new_id };
                    response_created(response)
                },
                Err(e) => response_error(e.as_str()),
            }
        },
        Err(e) => response_bad_request(&e),
    }
}

pub async fn update(State(state): State<AppState>, Json(data): Json<models::UpdateRequest>) -> impl IntoResponse {
    match data.to_entity() {
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
        Ok(entities) => {
            let models = entities.into_iter()
                .map(|entity| entity.into())
                .collect::<Vec<models::Custodian>>();
            response_ok(models)
        },
        Err(e) => response_error(e.as_str()),
    }
}
