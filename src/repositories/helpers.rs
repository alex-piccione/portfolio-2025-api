use rust_decimal::Decimal as RustDecimal;
use sqlx::{postgres::PgRow, types::BigDecimal, Error as SqlxError, Row};
use std::{str::FromStr};

use crate::repositories::errors::DatabaseError;

pub fn from_rust_decimal(rd: RustDecimal) -> Result<BigDecimal, String> {
    BigDecimal::from_str(&rd.to_string()).map_err(|e| format!("Failed to convert RustDecimal '{}' to BigDecimal. {}", rd, e))
}

pub fn to_rust_decimal(bd: BigDecimal) -> Result<RustDecimal, String> {
    RustDecimal::from_str(&bd.to_string()).map_err(|e| format!("Failed to convert BigDecimal '{}' to RustDecimal. {}", bd, e))
}

pub fn check_result_for_new_id(result: Result<PgRow, sqlx::Error>) -> Result<i32, DatabaseError> 
    {
    match result {
        Ok(row) => Ok(row.get(0)), // assumes id is the first column
        Err(SqlxError::Database(e)) => {
            match e.is_unique_violation() {
                true => Err(DatabaseError::duplicated_field(e.to_string())),
                _ => match e.code() {
                    Some(code) => Err(DatabaseError::generic(format!("Code: {}. {}", code, e.to_string()))),
                    None => Err(DatabaseError::generic(e.to_string())),
                },
            }
        },
        Err(err) => Err(DatabaseError::generic(err.to_string())),
    }  
}