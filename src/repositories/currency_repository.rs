use sqlx::PgPool;

use crate::entities::currency::Currency;
use crate::entities::currency::CurrencyKind;

pub struct CurrencyRepository {

}

impl CurrencyRepository {

    /*pub fn new() -> Self {
        CurrencyRepository {

        }
    }*/

    /*pub fn create(currency: Currency) -> Currency {
        currency
    }*/

    pub async fn list(db_pool: &PgPool) -> Result<Vec<Currency>, String> { //sqlx::Error
        //let currencies = sqlx::query_as!(Currency, "SELECT id, symbol, name, kind as \"kind: _\", is_active, precision FROM currencies")
        let currencies = sqlx::query_as!(Currency, 
            r#"
            SELECT 
                id, symbol, name, kind as "kind!: CurrencyKind", is_active, precision 
            FROM Currency
            "#)
            .fetch_all(db_pool)
            .await
            .map_err(|e:sqlx::Error| e.to_string())?;

        Ok(currencies)
    }
}