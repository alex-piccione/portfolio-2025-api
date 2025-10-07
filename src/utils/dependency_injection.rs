use sqlx::PgPool;

use crate::{configuration::Configuration, 
    repositories::{
        user_repository::UserRepository,
        session_repository::SessionRepository,
        currency_repository::CurrencyRepository, 
        custodian_repository::{CustodianRepository}, 
        }, 
    services::{
        user_service::UserService,
        session_service::SessionService,
        currency_service::CurrencyService,        
        custodian_service::CustodianService}};

#[derive(Clone)]
pub struct AppState {
    //pub config: Configuration,
    pub user_service: UserService,
    pub session_service: SessionService,
    pub currency_service: CurrencyService,
    pub custodian_service: CustodianService,
}

pub async fn inject_services(_config: &Configuration, db_pool:PgPool) -> AppState {

    let user_repository = UserRepository::new(db_pool.clone());
    let session_repository = SessionRepository::new(db_pool.clone());
    let currency_repository = CurrencyRepository::new(db_pool.clone());
    let custodian_repository = CustodianRepository::new(db_pool.clone());

    let currency_service = CurrencyService::new(currency_repository.clone());
    // **LOAD THE CACHE ONLY ONCE**
    if let Err(e) = currency_service.init_cache().await {
        eprintln!("FATAL: Failed to initialize currency cache: {}", e);
        std::process::exit(1); // Or handle the error as needed
    }

    let user_service = UserService::new(user_repository.clone(), currency_service.clone());

    AppState {
        //config: config.clone(),
        user_service: user_service.clone(),
        session_service: SessionService::new(session_repository.clone(), user_service.clone()),
        currency_service: currency_service.clone(),
        custodian_service: CustodianService::new(custodian_repository.clone()),
    }    
}

