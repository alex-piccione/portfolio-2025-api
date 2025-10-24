pub mod create {
    use crate::{entities::custodian::{Custodian, CustodianKind}};

    #[derive(serde::Deserialize)]
    #[serde(rename_all = "camelCase")] 
    pub struct Request {
        pub name: String,
        pub kind: String,
        pub description: Option<String>,
        pub url: Option<String>,
        pub wallet_address: Option<String>,
        pub account_country_code: Option<String>,
    }

    impl Request {
        pub fn to_entity(self) -> Result<Custodian, String> {
            Ok(Custodian {
                id: 0,
                name: self.name,
                kind: CustodianKind::from_string(&self.kind)?,
                description: self.description,
                url: self.url,
                wallet_address: self.wallet_address,
                account_country_code: self.account_country_code,
            })
        }
    }

    //pub type Response = NewIdResponse;
}

pub mod update {
    use serde::Deserialize;

    use crate::entities::custodian::{Custodian, CustodianKind};

    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")] 
    pub struct Request {
        pub id: i32,
        pub name: String,
        pub kind: String,
        pub description: Option<String>,
        pub url: Option<String>,
        pub wallet_address: Option<String>,
        pub account_country_code: Option<String>,
    }

    impl Request {
        pub fn to_entity(self) -> Result<Custodian, String> {
            Ok(Custodian {
                id: self.id,
                name: self.name,
                kind: CustodianKind::from_string(&self.kind)?,
                description: self.description,
                url: self.url,
                wallet_address: self.wallet_address,
                account_country_code: self.account_country_code,
            })
        }
    }

}