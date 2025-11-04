use axum::{middleware, routing::{get, post, put, delete}, Router};
use crate::{dependency_injection::AppState, endpoints};

pub fn set_routes(app_state: AppState) -> Router<AppState> {

    // Public routes
    let public_routes = Router::new()
        .route("/", get(endpoints::common_endpoint::home))        
        // auth
        .route("/auth/login", post(endpoints::auth_endpoint::login))
        .route("/auth/signup", post(endpoints::auth_endpoint::signup))
        .route("/auth/refresh", post(endpoints::auth_endpoint::refresh_token));

        // User required routes (without middleware applied yet)
    let user_routes = Router::new()
        // currency
        .route("/currency", post(endpoints::currency_endpoint::create))
        .route("/currency", put(endpoints::currency_endpoint::update))
        .route("/currency/{id}", get(endpoints::currency_endpoint::single))
        .route("/currency", get(endpoints::currency_endpoint::list_of_user))   
        .route("/currency/{id}/enable", put(endpoints::currency_endpoint::enable))  
        .route("/currency/{id}/disable", put(endpoints::currency_endpoint::disable))  
        // admin commands 
        .route("/currency/all", get(endpoints::currency_endpoint::list_all))   // TODO: put under admin commands 
        .route("/currency/{id}", delete(endpoints::currency_endpoint::delete))   // TODO: put under admin commands 
        // custodian
        .route("/custodian", post(endpoints::custodian_endpoint::create))
        .route("/custodian", put(endpoints::custodian_endpoint::update))
        .route("/custodian/{id}", delete(endpoints::custodian_endpoint::delete))
        .route("/custodian", get(endpoints::custodian_endpoint::list))    
        // holdings (todo)
        .route("/holding", post(endpoints::holding_endpoint::create))  
        .route("/holding/{id}", put(endpoints::holding_endpoint::update))  
        .route("/holding", get(endpoints::holding_endpoint::list))
        .route("/holding/{id}", delete(endpoints::holding_endpoint::delete));

    Router::new()
        .merge(public_routes)
        .merge(user_routes.layer(middleware::from_fn_with_state(
            app_state.clone(),
            crate::utils::auth_middleware::requires_user
        )))
        .with_state(app_state)
        
}

/* 
pub trait RequireUserExt {
    fn requires_user(self, state: AppState) -> Self;
}

impl RequireUserExt for Router<AppState> {
    fn requires_user(self, state: AppState) -> Self {
        self.layer(middleware::from_fn_with_state(state, crate::utils::auth_middleware::requires_user))
    }
}
*/
