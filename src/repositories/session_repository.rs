use sqlx::PgPool;

use crate::repositories::schemas::session_record::SessionRecord;

#[derive(Clone)]
pub struct SessionRepository {
    db_pool: PgPool,
}

impl SessionRepository {
    pub fn new(db_pool: PgPool) -> Self {
        Self {db_pool}
    }

    pub async fn create(&self, item: SessionRecord) -> Result<i32, String> {
        let row = sqlx::query!(
            r#"
            INSERT INTO Sessions (user_id, access_token, access_token_expires_at, refresh_token, refresh_token_expires_at, created_at, creation_ip_address, creation_user_agent)
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
                RETURNING id
            "#,
            item.user_id,
            item.access_token,
            item.access_token_expires_at,
            item.refresh_token,
            item.refresh_token_expires_at,
            item.created_at,
            item.creation_ip_address,
            item.creation_user_agent
        )
        .fetch_one(&self.db_pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(row.id)
    }

    pub async fn find_by_access_token(&self, access_token: &str) -> Result<Option<SessionRecord>, String> {
        //let _ = sqlx::query_as!(SessionRecord, "SELECT id, access_token, access_token_expires_at, refresh_token, refresh_token_expires_at  FROM Sessions");
        sqlx::query_as!(
            SessionRecord,
            r#"
            SELECT id, user_id, access_token, access_token_expires_at, refresh_token, refresh_token_expires_at, created_at, creation_ip_address, creation_user_agent
            FROM Sessions WHERE access_token = $1
            "#, 
            access_token)
                .fetch_optional(&self.db_pool)
                .await
                .map_err(|e| format!("Failed to get Session by access token. {}", e))
    }
}