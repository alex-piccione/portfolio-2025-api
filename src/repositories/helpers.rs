use rust_decimal::Decimal as RustDecimal;
use sqlx::{types::BigDecimal};
use std::{str::FromStr};

pub fn from_rust_decimal(rd: RustDecimal) -> Result<BigDecimal, String> {
    BigDecimal::from_str(&rd.to_string()).map_err(|e| format!("Failed to convert RustDecimal '{}' to BigDecimal. {}", rd, e))
}

pub fn to_rust_decimal(bd: BigDecimal) -> Result<RustDecimal, String> {
    RustDecimal::from_str(&bd.to_string()).map_err(|e| format!("Failed to convert BigDecimal '{}' to RustDecimal. {}", bd, e))
}
