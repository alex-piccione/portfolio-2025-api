use crate::{entities::custodian::Custodian, repositories::custodian_repository::CustodianRepository};

#[derive(Clone)]
pub struct CustodianService {
    repository: CustodianRepository
}

impl CustodianService {
    pub fn new(repository: CustodianRepository) -> Self {
        Self {repository}
    }

    pub async fn create(&self, item: Custodian) -> Result<i32, String> {
        self.repository.create(item).await
    }

    pub async fn update(&self, item: Custodian) -> Result<(), String> {
        self.repository.update(item).await
    }

    pub async fn list(&self) -> Result<Vec<Custodian>, String> {
        self.repository.list().await
    }    
}