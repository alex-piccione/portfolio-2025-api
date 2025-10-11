use crate::{entities::session::Session, utils::datetime::UtcDateTime};

#[derive(sqlx::FromRow)]
pub struct SessionRecord {
    #[allow(dead_code)]
    pub id: i32,
    pub user_id: String,
    pub access_token: String,
    pub access_token_expires_at: UtcDateTime,
    pub refresh_token: String,
    pub refresh_token_expires_at: UtcDateTime,
    pub created_at: UtcDateTime,
    pub creation_ip_address: String,
    pub creation_user_agent: String
}

impl From<Session> for SessionRecord {
    fn from(session: Session) -> Self {
        SessionRecord { 
            id: session.id, 
            user_id: session.user.id, 
            access_token: session.access_token, 
            access_token_expires_at: session.access_token_expires_at, 
            refresh_token: session.refresh_token, 
            refresh_token_expires_at: session.refresh_token_expires_at, 
            created_at: session.created_at, 
            creation_ip_address: session.creation_ip_address, 
            creation_user_agent: session.creation_user_agent }
    }
}

#[derive(sqlx::FromRow)]
pub struct SessionWithUser {
    pub user_id: String,
    pub username: String,
    pub access_token_expires_at: UtcDateTime,
    pub refresh_token_expires_at: UtcDateTime,
}

pub struct UpdateForAccess {
    pub access_token: String,
    pub access_token_expires_at: UtcDateTime,
    pub refresh_token_expires_at: UtcDateTime
    // TODO: last_access_at
}

pub struct UpdateForRefresh {
    pub old_refresh_token: String,
    pub access_token: String,
    pub access_token_expires_at: UtcDateTime,
    pub refresh_token: String,
    pub refresh_token_expires_at: UtcDateTime
    // TODO: last_refresh_at
}