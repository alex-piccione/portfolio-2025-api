use crate::repositories::currency_rate_repository::CurrencyRateRepository;
use crate::repositories::schemas::currency_rate_record::CurrencyRateRecord;
use crate::utils::datetime::UtcDateTime;

#[derive(Clone)]
pub struct CurrencyRateService {
    repository: CurrencyRateRepository,
}

impl CurrencyRateService {
    pub fn new(repository: CurrencyRateRepository) -> Self {
        Self { repository }
    }

    pub async fn create(&self, record: CurrencyRateRecord) -> Result<(), String> {
        self.repository.create(record).await
    }

    pub async fn search(&self, base_currency_id: i32, quote_currency_id: i32, date: Option<UtcDateTime>) -> Result<Vec<CurrencyRateRecord>, String> {
        self.repository.search(base_currency_id, quote_currency_id, date).await
    }

    pub async fn list_at_date(&self, date: UtcDateTime) -> Result<Vec<CurrencyRateRecord>, String> {
        self.repository.list_at_date(date).await
    }
}