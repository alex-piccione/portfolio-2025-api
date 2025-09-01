use sqlx::{FromRow, Type};

#[derive(FromRow, Debug, Clone)]
pub struct Custodian {
    pub id: i32,
    pub name: String,
    pub kind: CustodianKind,
    pub description: Option<String>,
    pub url: Option<String>,
    pub wallet_address: Option<String>,
    pub country_code: Option<String>,
}

#[derive(Debug, Clone, Type)]
#[sqlx(type_name = "VARCHAR")]
pub enum CustodianKind {
    Bank,
    Exchange,
    Pension,
    #[sqlx(rename = "Blockchain Wallet")] // required if we use "BlockchainWallet" (no underscore)
    BlockchainWallet,
}