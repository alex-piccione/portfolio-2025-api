use axum::{extract::State, http::StatusCode, Json};
use axum::response::IntoResponse;
use crate::endpoints::helpers::{response_error, response_ok};
use crate::AppState;
use crate::endpoints::models::custodian as models;

pub async fn create(State(state): State<AppState>, Json(data): Json<models::CreateRequest>) -> impl IntoResponse {
    match data.to_entity() {
        Ok(entity) => {
            match state.custodian_repository.create(entity).await {
                Ok(new_id) => {
                    let response = models::CreateResponse { new_id };
                    response_ok(response)
                },
                Err(e) => response_error(StatusCode::INTERNAL_SERVER_ERROR, e.as_str()),
            }
        },
        Err(e) => response_error(StatusCode::BAD_REQUEST, e.as_str()),
    }
}

pub async fn list(State(state): State<AppState>) -> impl IntoResponse {
    match state.custodian_repository.list().await {
        Ok(entities) => {
            let models = entities.into_iter()
                .map(|entity| entity.into())
                .collect::<Vec<models::Custodian>>();   
            response_ok(models)
        },
        Err(e) => response_error(StatusCode::INTERNAL_SERVER_ERROR, e.as_str()),
    }
}
