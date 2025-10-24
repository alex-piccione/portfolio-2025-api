use sqlx::{PgPool};
use crate::{entities::custodian::{Custodian, CustodianKind}, repositories::errors::{DatabaseError}};

#[derive(Clone)]
pub struct CustodianRepository {
    db_pool: PgPool,
}

impl CustodianRepository {
    pub fn new(db_pool: PgPool) -> Self {
        Self { db_pool }
    }

    pub async fn create(&self, custodian: Custodian) -> Result<i32, DatabaseError> {
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

        match result {
            Ok(row) => Ok(row.id),
            /*
            Err(error) =>  {
                // 23505 is the error code for unique_violation and Name is the only unique field at the moment
                match error.code() {
                    Some("23505") => Err(RepositoryError::DuplicatedField(error.to_string())),
                    _ => Err(RepositoryError::DatabaseError(error.to_string()))
                }
            },
            */
            // All other errors
            //Err(err) => Err(RepositoryError::UnexpectedError(err.into())),
            Err(err) => Err(DatabaseError::generic(err.to_string())),
        }        
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




