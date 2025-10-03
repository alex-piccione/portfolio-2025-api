use sqlx::PgPool;

use crate::{configuration::Configuration, 
    repositories::{
        currency_repository::CurrencyRepository, 
        custodian_repository::CustodianRepository, 
        user_repository::UserRepository}, 
    services::{
        currency_provider::CurrencyProvider, 
        currency_service::CurrencyService, 
        user_service::UserService}};

#[derive(Clone)]
pub struct AppState {
    pub config: Configuration,
    pub user_service: UserService,
    pub currency_service: CurrencyService,
    pub currency_repository: CurrencyRepository,
    pub custodian_repository: CustodianRepository,
}

pub async fn inject_services(config: &Configuration, db_pool:PgPool) -> AppState {
    
    let currency_repository = CurrencyRepository::new(db_pool.clone());
    if let Err(e) = CurrencyProvider::load(&currency_repository).await {
        eprintln!("Failed to load currencies: {}", e);
        std::process::exit(1);
    }

    let user_repository = UserRepository::new(db_pool.clone());

    AppState {
        config: config.clone(),
        user_service: UserService::new(user_repository.clone()),
        currency_service: CurrencyService::new(currency_repository.clone()),
        currency_repository: currency_repository.clone(),
        custodian_repository: CustodianRepository::new(db_pool),
    }    
}

