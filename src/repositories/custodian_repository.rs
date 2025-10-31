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
                INSERT INTO Custodians (user_id, name, custodian, account, kind, color_code, description)
                VALUES ($1, $2, $3, $4, $5, $6, $7)
                RETURNING id
            "#
        )
        .bind(&custodian.user_id)
        .bind(&custodian.name)
        .bind(&custodian.custodian)
        .bind(&custodian.account)
        .bind(&custodian.kind)
        .bind(&custodian.color_code)
        .bind(&custodian.description)
        .fetch_one(&self.db_pool)
        .await;

        check_result_for_new_id(result)
    }

    pub async fn update(&self, custodian: Custodian) -> Result<(), String> {
        sqlx::query!(
            r#"
                UPDATE Custodians
                SET name = $2, custodian = $3, account = $4, kind = $5, color_code = $6, description = $7
                WHERE id = $1
            "#,
            custodian.id,
            custodian.name,
            custodian.custodian,
            custodian.account,
            custodian.kind as CustodianKind,
            custodian.color_code,
            custodian.description
        )
        .execute(&self.db_pool)
        .await
        .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub async fn list(&self) -> Result<Vec<Custodian>, String> {
        let custodians = sqlx::query_as!(Custodian,
            r#"
                SELECT id, user_id, name, custodian, account, kind as "kind!: CustodianKind", color_code, description
                FROM Custodians
            "#
        )
        .fetch_all(&self.db_pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(custodians)
    }
}




