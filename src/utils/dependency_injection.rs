use sqlx::PgPool;

use crate::{configuration::Configuration, 
    repositories::{
        currency_repository::CurrencyRepository, 
        custodian_repository::{CustodianRepository}, 
        user_repository::UserRepository}, 
    services::{
        currency_service::CurrencyService, 
        user_service::UserService,
        custodian_service::CustodianService}};

#[derive(Clone)]
pub struct AppState {
    //pub config: Configuration,
    pub user_service: UserService,
    pub currency_service: CurrencyService,
    pub custodian_service: CustodianService,
}

pub async fn inject_services(_config: &Configuration, db_pool:PgPool) -> AppState {

    let user_repository = UserRepository::new(db_pool.clone());
    let currency_repository = CurrencyRepository::new(db_pool.clone());
    let custodian_repository = CustodianRepository::new(db_pool.clone());

    let currency_service = CurrencyService::new(currency_repository.clone());
    // **LOAD THE CACHE ONLY ONCE**
    if let Err(e) = currency_service.init_cache().await {
        eprintln!("FATAL: Failed to initialize currency cache: {}", e);
        std::process::exit(1); // Or handle the error as needed
    }

    AppState {
        //config: config.clone(),
        user_service: UserService::new(user_repository.clone(), currency_service.clone()),
        currency_service: currency_service.clone(),
        custodian_service: CustodianService::new(custodian_repository.clone()),
    }    
}

