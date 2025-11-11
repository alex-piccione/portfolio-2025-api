use rust_decimal::Decimal;
use crate::utils::datetime::UtcDateTime;

#[derive(sqlx::FromRow, serde::Serialize)]
#[serde(rename_all = "camelCase")] 
pub struct HoldingRecord {
    pub id: i32,
    
    pub user_id: String,
    pub custodian_id: i32,
    pub currency_id: i32,

    pub date: UtcDateTime,
    pub action: String,
    pub amount: Decimal,
    pub note: Option<String>,
}
