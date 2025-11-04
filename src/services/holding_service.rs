use crate::{ endpoints::models::holding_models::{create, update}, 
    repositories::{custodian_repository::CustodianRepository, errors::DatabaseError, holding_repository::HoldingRepository, schemas::holding_record::HoldingRecord}, services::currency_service::CurrencyService};

#[derive(Clone)]
pub struct HoldingService {
    repository: HoldingRepository, 
    _currency_service: CurrencyService,
    _custodian_repository: CustodianRepository
}

impl HoldingService {
    pub fn new(repository: HoldingRepository, _currency_service: CurrencyService, _custodian_repository: CustodianRepository) -> Self {
        Self {repository, _currency_service, _custodian_repository}
    }

    pub async fn create(&self, user_id: &str, request: create::Request) -> Result<i32, String> {
        let record: HoldingRecord = (request, user_id).into();
        self.repository.create(record).await
    }

    pub async fn update(&self, user_id: &str, id: i32, request: update::Request) -> Result<(), DatabaseError> {
        let record: HoldingRecord = (id, request, user_id).into();
        self.repository.update(record).await
    }

    pub async fn delete(&self, user_id: &str, id: i32) -> Result<(), DatabaseError> {
        self.repository.delete(id, &user_id).await
    }

    pub async fn single_for_user(&self, user_id:&str, id:i32) -> Result<HoldingRecord, String> {
        self.repository.single_for_user(id, user_id).await
    }

    pub async fn list_for_user(&self, user_id:&str) -> Result<Vec<HoldingRecord>, String> {
        self.repository.list(user_id).await
        /*let records = self.repository.list(&user.id).await?;        
        let custodians = self.custodian_repository.list().await?;

        records.into_iter().map(|record| -> Result<Holding, String> {
            Ok(Holding {
                id: record.id,
                user: user.clone(),
                custodian: find_by!(custodians, |item| item.id == record.custodian_id)?,
                currency: self.currency_service.get(record.currency_id),
                date: record.date,
                action: record.action,
                amount: record.amount,
                note: record.note
            })
        })        
        .to_vec()  // instead of .collect::<Result<Vec<_>, _>>()
        */
    }    
}