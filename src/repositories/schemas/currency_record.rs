/*
#[derive(sqlx::FromRow, serde::Serialize)]
#[serde(rename_all = "camelCase")] 
pub struct CurrencyRecord {
    pub id: i32,
    
    pub user_id: String,
    pub custodian_id: i32,
    pub currency_id: i32,

    pub date: UtcDateTime,
    pub action: String,
    pub amount: Decimal,
    pub note: Option<String>,    
}
*/

#[derive(sqlx::FromRow, serde::Serialize)]
#[serde(rename_all = "camelCase")] 
pub struct CurrencyOfUserRecord {
    pub id: i32,    
    pub user_id: String,
    pub currency_id: i32,
}
