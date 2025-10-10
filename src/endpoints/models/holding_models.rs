pub mod create {
    use rust_decimal::Decimal;
    use crate::{repositories::schemas::holding_record::HoldingRecord, utils::datetime::UtcDateTime};

    #[derive(serde::Deserialize)]
    #[serde(rename_all = "camelCase")] 
    pub struct Request {
        pub custodian_id: i32,
        pub currency_id: i32,

        pub date: UtcDateTime,
        pub action: String,
        pub amount: Decimal,
        pub note: Option<String>,            
    }

    impl From<(Request, &str)> for HoldingRecord {
        fn from((request, user_id): (Request, &str)) -> HoldingRecord {
            HoldingRecord { 
                id: 0, 
                user_id: user_id.to_string(),
                custodian_id: request.custodian_id, 
                currency_id: request.currency_id, 
                date: request.date, 
                action: request.action, 
                amount: request.amount, 
                note: request.note
            }
        }
    }
}