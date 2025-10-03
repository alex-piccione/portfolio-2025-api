use crate::{entities::currency::Currency, repositories::currency_repository::CurrencyRepository};

#[derive(Clone)]
pub struct CurrencyService {
    repository: CurrencyRepository
}

impl CurrencyService {
    pub fn new(repository: CurrencyRepository) -> Self {
        Self { repository }
    }

    pub async fn create(&self, item:Currency) -> Result<i32, String> {
        self.repository.create(item).await
    }

    pub async fn update(&self, item: Currency) -> Result<(), String> {
        self.repository.update(item).await
    }

    pub async fn list(&self) -> Result<Vec<Currency>, String> {
        self.repository.list().await
    }
}