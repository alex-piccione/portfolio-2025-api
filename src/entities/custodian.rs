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

impl CustodianKind {
    pub fn from_string(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "Bank" => Ok(CustodianKind::Bank),
            "Exchange" => Ok(CustodianKind::Exchange),
            "Pension" => Ok(CustodianKind::Pension),
            "Blockchain Wallet" => Ok(CustodianKind::BlockchainWallet),
            _ => Err(format!("Invalid custodian kind: {}", s)),
        }
    }
}

impl From<CustodianKind> for String {
    fn from(kind: CustodianKind) -> Self {
        match kind {
            CustodianKind::Bank => "Bank".to_string(),
            CustodianKind::Exchange => "Exchange".to_string(),
            CustodianKind::Pension => "Pension".to_string(),
            CustodianKind::BlockchainWallet => "Blockchain Wallet".to_string(),
        }
    }
}