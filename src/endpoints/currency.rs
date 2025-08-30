use axum::{extract::Path, http::StatusCode, Json};
use axum::response::{IntoResponse};

use super::helpers::{response_error, response_ok};
use crate::endpoints::{currency_models as models};
//use crate::entities;


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

pub async fn list() -> impl IntoResponse {
    let data = vec![
        models::Currency {
            id: 1,
            symbol: "USD".to_string(),
            name: "United States Dollar".to_string(),
            kind: "Fiat".to_string(),
            is_active: true,
            precision: 2,
        },        
        models::Currency {
            id: 2,
            symbol: "EUR".to_string(),
            name: "Euro".to_string(),
            kind: "Fiat".to_string(),
            is_active: true,
            precision: 2,
        },
    ];

    Json(data)
}

pub async fn create (Json(data): Json<models::CreateRequest>) -> impl IntoResponse {

    match data.to_entity() {
        Ok(entity) => {
            // TODO save the currency to a database
            // it will generate the ID
            let response = models::CreateResponse {new_id: entity.id};
            response_ok(response)
        },
        Err(e) => response_error(StatusCode::BAD_REQUEST,  e.as_str())       
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
