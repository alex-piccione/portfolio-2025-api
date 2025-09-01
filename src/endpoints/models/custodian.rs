use serde::{Deserialize, Serialize};
use crate::entities::custodian as entities;
use crate::entities::custodian::CustodianKind;

#[derive(Serialize)]
pub struct Custodian {
    pub id: i32,
    pub name: String,
    pub kind: String,
    pub description: Option<String>,
    pub url: Option<String>,
    pub wallet_address: Option<String>,
    pub country_code: Option<String>,
}

impl From<entities::Custodian> for Custodian {
    fn from(entity: entities::Custodian) -> Self {
        Custodian {
            id: entity.id,
            name: entity.name,
            kind: entity.kind.into(),
            description: entity.description,
            url: entity.url,
            wallet_address: entity.wallet_address,
            country_code: entity.country_code,
        }
    }
}

#[derive(Deserialize)]
pub struct CreateRequest {
    pub name: String,
    pub kind: String,
    pub description: Option<String>,
    pub url: Option<String>,
    pub wallet_address: Option<String>,
    pub country_code: Option<String>,
}

#[derive(Serialize)]
pub struct CreateResponse {
    pub new_id: i32,
}

impl CreateRequest {
    pub fn to_entity(self) -> Result<entities::Custodian, String> {
        Ok(entities::Custodian {
            id: 0,
            name: self.name,
            kind: CustodianKind::from_string(&self.kind)?,
            description: self.description,
            url: self.url,
            wallet_address: self.wallet_address,
            country_code: self.country_code,
        })
    }
}
