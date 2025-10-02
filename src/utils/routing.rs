use axum::{routing::{get, post, put}, Router};
use crate::{endpoints, AppState};

pub fn set_routes(router:Router<AppState>) -> Router<AppState> {
    router
        .route("/", get(endpoints::common::home))        
        // auth
        .route("/login", get(endpoints::auth::login))
        .route("/signup", post(endpoints::auth::signup))
        // currency
        .route("/currency", post(endpoints::currency::create))
        .route("/currency", put(endpoints::currency::update))
        .route("/currency/{id}", get(endpoints::currency::single))
        .route("/currency", get(endpoints::currency::list))        
        // custodian
        .route("/custodian", post(endpoints::custodian::create))
        .route("/custodian", put(endpoints::custodian::update))
        .route("/custodian", get(endpoints::custodian::list))        
        // holdings
}

