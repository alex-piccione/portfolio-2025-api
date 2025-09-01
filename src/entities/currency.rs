use sqlx::FromRow;
use sqlx::Type;

#[derive(Debug, Clone, FromRow)]
pub struct Currency {
    pub id: i32,
    pub symbol: String,
    pub name: String,
    pub kind: CurrencyKind,
    pub is_active: bool,
    pub precision: i16,    
}

#[derive(Debug, Clone, Type)]
#[sqlx(type_name = "VARCHAR")]
pub enum CurrencyKind {
    Fiat,
    Crypto,
    Stablecoin,
}

impl CurrencyKind {
    pub fn from_string(kind: &str) -> Result<Self, String> {
        match kind {
            "Fiat" => Ok(CurrencyKind::Fiat),
            "Crypto" => Ok(CurrencyKind::Crypto),
            "Stablecoin" => Ok(CurrencyKind::Stablecoin),
            _ => Err(format!("Invalid currency kind: {}", kind)),
        }
    }
}

impl TryFrom<String> for CurrencyKind {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        CurrencyKind::from_string(&value)
    }
}

impl From<CurrencyKind> for String {
    fn from(kind: CurrencyKind) -> Self {
        match kind {
            CurrencyKind::Fiat => "Fiat".to_string(),
            CurrencyKind::Crypto => "Crypto".to_string(),
            CurrencyKind::Stablecoin => "Stablecoin".to_string(),
        }
    }
}
