use sqlx::{ PgPool};
use crate::{entities::custodian::{Custodian, CustodianKind}, repositories::{errors::DatabaseError, helpers::check_result_for_new_id}};

#[derive(Clone)]
pub struct CustodianRepository {
    db_pool: PgPool,
}

impl CustodianRepository {
    pub fn new(db_pool: PgPool) -> Self {
        Self { db_pool }
    }

    pub async fn create(&self, custodian: Custodian) -> Result<i32, DatabaseError> {
        /*  this returns an anonymous struct  a dynamic 
        let result = sqlx::query!(
            r#"
                INSERT INTO Custodian (name, kind, description, url, wallet_address, account_country_code)
                VALUES ($1, $2, $3, $4, $5, $6)
                RETURNING id
            "#,
            custodian.name,
            custodian.kind as CustodianKind,
            custodian.description,
            custodian.url,
            custodian.wallet_address,
            custodian.account_country_code
        )
        .fetch_one(&self.db_pool)
        .await;
        */
        let result = sqlx::query(
            r#"
                INSERT INTO Custodian (name, kind, description, url, wallet_address, account_country_code)
                VALUES ($1, $2, $3, $4, $5, $6)
                RETURNING id
            "#
        )
        .bind(&custodian.name)
        .bind(&custodian.kind)
        .bind(&custodian.description)
        .bind(&custodian.url)
        .bind(&custodian.wallet_address)
        .bind(&custodian.account_country_code)
        .fetch_one(&self.db_pool)
        .await;

        check_result_for_new_id(result)
    }

    pub async fn update(&self, custodian: Custodian) -> Result<(), String> {
        sqlx::query!(
            r#"
                UPDATE Custodian
                SET name = $2, kind = $3, description = $4, url = $5, wallet_address = $6, account_country_code = $7
                WHERE id = $1
            "#,
            custodian.id,
            custodian.name,
            custodian.kind as CustodianKind,
            custodian.description,
            custodian.url,
            custodian.wallet_address,
            custodian.account_country_code
        )
        .execute(&self.db_pool)
        .await
        .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub async fn list(&self) -> Result<Vec<Custodian>, String> {
        let custodians = sqlx::query_as!(Custodian,
            r#"
                SELECT id, name, kind as "kind!: CustodianKind", description, url, wallet_address, account_country_code
                FROM Custodian
            "#
        )
        .fetch_all(&self.db_pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(custodians)
    }
}




