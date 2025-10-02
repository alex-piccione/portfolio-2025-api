use sqlx::PgPool;
use crate::entities::user::User;


#[derive(Clone)]
pub struct UserRepository {
    db_pool: PgPool,
}

impl UserRepository {
    pub fn new(db_pool: PgPool) -> Self {
        Self { db_pool }
    }

    pub async fn create(&self, user: User) -> Result<(), String> {

        sqlx::query!(
            r#"
                INSERT INTO Users (id, username, hashed_password, creation_date, currency_id )
                VALUES ($1, $2, $3, $4, $5)
            "#,
            user.id,
            user.username,
            user.hashed_password,
            user.creation_date.as_timestamptz(),
            user.currency.id
        )
        .execute(&self.db_pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(())
    }
}