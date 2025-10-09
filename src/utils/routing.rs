use axum::{middleware, routing::{get, post, put}, Router};
use crate::{endpoints, dependency_injection::AppState};

pub fn set_routes(app_state: AppState) -> Router<AppState> {

    // Public routes
    let public_routes = Router::new()
        .route("/", get(endpoints::common_endpoint::home))        
        // auth
        .route("/login", post(endpoints::auth_endpoint::login))
        .route("/signup", post(endpoints::auth_endpoint::signup));

    // User required routes
    let user_routes = Router::new()
        // currency
        .route("/currency", post(endpoints::currency_endpoint::create))
        .route("/currency", put(endpoints::currency_endpoint::update))
        .route("/currency/{id}", get(endpoints::currency_endpoint::single))
        .route("/currency", get(endpoints::currency_endpoint::list))        
        // custodian
        .route("/custodian", post(endpoints::custodian_endpoint::create))
        .route("/custodian", put(endpoints::custodian_endpoint::update))
        .route("/custodian", get(endpoints::custodian_endpoint::list))    
        // holdings (todo)
            .requires_user(app_state.clone());

    Router::new()
        .merge(public_routes)
        .merge(user_routes)
        .with_state(app_state)
        
}

pub trait RequireUserExt {
    fn requires_user(self, state: AppState) -> Self;
}

impl RequireUserExt for Router<AppState> {
    fn requires_user(self, state: AppState) -> Self {
        self.layer(middleware::from_fn_with_state(state, crate::utils::auth_middleware::requires_user))
    }
}
