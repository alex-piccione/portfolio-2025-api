//pub(crate) struct Currency {
pub struct Currency {
    pub id: i32,
    pub symbol: String,
    pub name: String,
    pub kind: CurrencyKind,
    pub is_active: bool,
    pub precision: i8,    
}

pub enum CurrencyKind {
    Fiat,
    Crypto,
    Stablecoin,
}