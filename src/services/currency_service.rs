use crate::{
    entities::currency::Currency, 
    repositories::currency_repository::CurrencyRepository};

#[derive(Clone)]
pub struct CurrencyService {
    currency_repository: CurrencyRepository
}

impl CurrencyService {
    pub fn new(currency_repository: CurrencyRepository) -> Self {
        Self { currency_repository }
    }

    pub async fn create(&self, currency:Currency) -> Result<i32, String> {
        self.currency_repository.create(currency).await
    }

    pub async fn update(&self, currency: Currency) -> Result<(), String> {
        self.currency_repository.update(currency).await
    }

    pub async fn list(&self) -> Result<Vec<Currency>, String> {
        self.currency_repository.list().await
    }
}