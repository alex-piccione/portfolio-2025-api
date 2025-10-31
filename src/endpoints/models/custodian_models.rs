pub mod create {
    use crate::{entities::custodian::{Custodian, CustodianKind}};

    #[derive(serde::Deserialize)]
    #[serde(rename_all = "camelCase")] 
    pub struct Request {
        pub name: String,
        pub custodian: String,
        pub account: Option<String>,
        pub kind: String,
        pub color_code: String,
        pub description: Option<String>
    }

    impl Request {
        pub fn to_entity(self, user_id: String) -> Result<Custodian, String> {
            Ok(Custodian {
                id: 0,
                user_id,
                name: self.name,
                custodian: self.custodian,
                account: self.account,
                kind: CustodianKind::from_string(&self.kind)?,
                color_code: self.color_code,
                description: self.description
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
        pub custodian: String,
        pub account: Option<String>,
        pub kind: String,
        pub color_code: String,
        pub description: Option<String>,
    }

    impl Request {
        pub fn to_entity(self, user_id:String) -> Result<Custodian, String> {
            Ok(Custodian {
                id: self.id,
                user_id,
                name: self.name,
                custodian: self.custodian,
                account: self.account,
                kind: CustodianKind::from_string(&self.kind)?,
                color_code: self.color_code,
                description: self.description
            })
        }
    }

}