use rust_decimal::Decimal;
use crate::{entities::{currency::Currency, custodian::Custodian, user::User}, repositories::schemas::holding_record::HoldingRecord, utils::datetime::UtcDateTime};

pub struct Holding {
    pub id: i32,
    
    pub user: User,
    pub custodian: Custodian,
    pub currency: Currency,

    pub date: UtcDateTime,
    pub action: String,
    pub amount: Decimal,
    pub note: Option<String>,    
}

impl From<Holding> for HoldingRecord {
    fn from(entity: Holding) -> Self {
        Self {
            id: entity.id,
            user_id: entity.user.id,
            custodian_id: entity.custodian.id,
            currency_id: entity.currency.id,
            date: entity.date,
            action: entity.action,
            amount: entity.amount,
            note: entity.note
        }
    }
}