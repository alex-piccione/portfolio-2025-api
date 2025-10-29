use axum::extract::Path;
use axum::Extension;
use axum::{extract::State};
use axum::response::IntoResponse;
use crate::endpoints::request_json_validator::ValidJson;
use crate::endpoints::response_utils::*;
use crate::dependency_injection::AppState;
use crate::endpoints::models::holding_models as models;
use crate::repositories::errors::{ErrorKind};
use crate::utils::auth_middleware::Session;

pub async fn create(
    State(state): State<AppState>, 
    Extension(session): Session,
    ValidJson(request): ValidJson<models::create::Request>) -> impl IntoResponse {

    // TODO: validation

    match state.holding_service.create(&session.user_id, request).await {
        Ok(new_id) => response_created_new_id(new_id),
        Err(e) => response_error(&e)
    }
}

/* 
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
*/

pub async fn delete(
    State(state): State<AppState>, 
    Extension(session): Session,
    Path(id):Path<i32>) -> impl IntoResponse {

    // TODO: validation

    match state.holding_service.delete(&session.user_id, id).await {
        Ok(()) => response_ok(()),
        Err(e) if e.kind == ErrorKind::RecordNotFound => {
            response_not_found(&e.message)
        },
        Err(e)  => response_error(&e.message)
    }
}


pub async fn list(
    State(state): State<AppState>, 
    Extension(session): Session) -> impl IntoResponse {
    match state.holding_service.list_for_user(&session.user_id).await {
        Ok(entities) => response_ok(entities),
        /*{
            let models = entities.into_iter()
                .map(|entity| entity.into())
                .collect::<Vec<models::Custodian>>();            
        },*/
        Err(e) => response_error(e.as_str()),
    }
}

