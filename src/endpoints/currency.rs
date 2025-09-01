use axum::{extract::Path,  extract::State, http::StatusCode, Json, response::IntoResponse};

use super::helpers::{response_ok, response_error};
use crate::AppState;
use crate::endpoints::{currency_models as models};

pub async fn create(State(state): State<AppState>, Json(data): Json<models::CreateRequest>) -> impl IntoResponse {

    match data.to_entity() {
        Ok(entity) => {
            match state.currency_repository.create(entity).await {
                Ok(new_id) => {
                    let response = models::CreateResponse { new_id };
                    response_ok(response)
                },
                Err(e) => response_error(StatusCode::INTERNAL_SERVER_ERROR, e.as_str())
            }
        },
        Err(e) => response_error(StatusCode::BAD_REQUEST,  e.as_str())       
    }
}

pub async fn update(State(state): State<AppState>, Json(data): Json<models::UpdateRequest>) -> impl IntoResponse {
    match data.to_entity() {
        Ok(entity) => {
            match state.currency_repository.update(entity).await {
                Ok(()) => response_ok("Currency updated successfully"),
                Err(e) => response_error(StatusCode::INTERNAL_SERVER_ERROR, e.as_str()),
            }
        },
        Err(e) => response_error(StatusCode::BAD_REQUEST, e.as_str()),
    }
}

pub async fn single(_id:Path<i32>) -> impl IntoResponse {

    let data = models::Currency {
        id: 1,
        symbol: "USD".to_string(),
        name: "United States Dollar".to_string(),
        kind: "Fiat".to_string(),
        is_active: true,
        precision: 2,
    };

    Json(data)
}

pub async fn list(State(state): State<AppState>) -> impl IntoResponse {

    match state.currency_repository.list().await {
        Ok(entities) => {

            let models = entities.into_iter()
                .map(|entity| entity.into())
                .collect::<Vec<models::Currency>>();

            // more "explicit" version compared to the idiomatic use of .into() above
            /*
            let models = entities.into_iter()
                .map(|e| models::Currency::from(e))
                .collect::<Vec<models::Currency>>();
            */

            response_ok(models)
        },
        Err(e) => response_error(StatusCode::INTERNAL_SERVER_ERROR, e.as_str())
    }
}



/* 
        // Handle the error case
        axum::response::Response::builder()
            .status(400)
            .body(axum::body::Body::from(e))
            .unwrap()        
    */


/* .map_err(|e| {
    axum::response::Response::builder()
        .status(400)
        .body(axum::body::Body::from(e))
        .unwrap()
})?;*/
