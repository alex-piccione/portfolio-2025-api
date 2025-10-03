use axum::{routing::{get, post, put}, Router};
use crate::{endpoints, dependency_injection::AppState};

pub fn set_routes(router:Router<AppState>) -> Router<AppState> {
    router
        .route("/", get(endpoints::common_endpoint::home))        
        // auth
        .route("/login", get(endpoints::auth_endpoint::login))
        .route("/signup", post(endpoints::auth_endpoint::signup))
        // currency
        .route("/currency", post(endpoints::currency_endpoint::create))
        .route("/currency", put(endpoints::currency_endpoint::update))
        .route("/currency/{id}", get(endpoints::currency_endpoint::single))
        .route("/currency", get(endpoints::currency_endpoint::list))        
        // custodian
        .route("/custodian", post(endpoints::custodian_endpoint::create))
        .route("/custodian", put(endpoints::custodian_endpoint::update))
        .route("/custodian", get(endpoints::custodian_endpoint::list))        
        // holdings
}

