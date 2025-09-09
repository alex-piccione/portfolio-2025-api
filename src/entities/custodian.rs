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
    #[sqlx(rename = "Fintech Platform")]
    FintechPlatform,
    Pension,
    #[sqlx(rename = "Blockchain Wallet")]
    BlockchainWallet,
}

impl CustodianKind {
    pub fn from_string(kind: &str) -> Result<Self, String> {
        match kind {
            "Bank" => Ok(CustodianKind::Bank),
            "Exchange" => Ok(CustodianKind::Exchange),
            "Fintech Platform" => Ok(CustodianKind::FintechPlatform), // Alias
            "Pension" => Ok(CustodianKind::Pension),
            "Blockchain Wallet" => Ok(CustodianKind::BlockchainWallet),
            _ => Err(format!("Invalid custodian kind: '{}'", kind)),
        }
    }
}

impl From<CustodianKind> for String {
    fn from(kind: CustodianKind) -> Self {
        match kind {
            CustodianKind::Bank => "Bank".to_string(),
            CustodianKind::Exchange => "Exchange".to_string(),
            CustodianKind::FintechPlatform => "Fintech Platform".to_string(),
            CustodianKind::Pension => "Pension".to_string(),
            CustodianKind::BlockchainWallet => "Blockchain Wallet".to_string(),
        }
    }
}