use sqlx::PgPool;

use crate::entities::custodian::{Custodian, CustodianKind};

#[derive(Clone)]
pub struct CustodianRepository {
    db_pool: PgPool,
}

impl CustodianRepository {
    pub fn new(db_pool: PgPool) -> Self {
        Self { db_pool }
    }

    pub async fn create(&self, custodian: Custodian) -> Result<i32, String> {
        let row = sqlx::query!(
            r#"
                INSERT INTO Custodian (name, kind, description, url, wallet_address, country_code)
                VALUES ($1, $2, $3, $4, $5, $6)
                RETURNING id
            "#,
            custodian.name,
            custodian.kind as CustodianKind,
            custodian.description,
            custodian.url,
            custodian.wallet_address,
            custodian.country_code
        )
        .fetch_one(&self.db_pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(row.id)
    }

    pub async fn list(&self) -> Result<Vec<Custodian>, String> {
        let custodians = sqlx::query_as!(Custodian,
            r#"
                SELECT id, name, kind as "kind!: CustodianKind", description, url, wallet_address, country_code
                FROM Custodian
            "#
        )
        .fetch_all(&self.db_pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(custodians)
    }
}
