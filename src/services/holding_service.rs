use crate::{entities::{user::User}, repositories::{custodian_repository::CustodianRepository, holding_repository::HoldingRepository, schemas::holding_record::HoldingRecord}, services::currency_service::CurrencyService};
use crate::endpoints::models::holding_models::create::Request as CreateRequest;

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

    pub async fn create(
        &self, user_id: &str,
        request: CreateRequest) -> Result<i32, String> {

        let record: HoldingRecord = (request, user_id).into();

        self.repository.create(record).await
    }

    /*pub async fn update(&self, item: Holding) -> Result<(), String> {
        self.repository.update(item).await
    }*/

    pub async fn list_for_user(&self, user:User) -> Result<Vec<HoldingRecord>, String> {
        self.repository.list(&user.id).await
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