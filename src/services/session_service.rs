use chrono::{DateTime, Utc};
//use rand::rngs::OsRng;
use rand::RngCore;
use base64::{engine::general_purpose, Engine as _};
//use sqlx::types::time::OffsetDateTime;

use crate::{constants, 
    entities::{session::Session, user::User},
    repositories::{schemas::session_record::SessionRecord, session_repository::SessionRepository},
    services::user_service::UserService, utils::datetime
};

#[derive(Clone)]
pub struct SessionService {
    repository: SessionRepository,
    user_service: UserService
}

pub fn generate_token() -> String {
    let mut bytes = [0u8; 48];
    rand::rng().fill_bytes(&mut bytes);
    general_purpose::URL_SAFE_NO_PAD.encode(bytes)
}

impl SessionService {

    pub fn new(repository: SessionRepository, user_service: UserService) -> Self {
        Self { repository, user_service }
    }

    pub async fn create(&self, user_id: String, ip_address: String, user_agent: String) -> Result<i32, String> {

        let now = datetime::now();
        let access_expires_at = now + constants::ACCESS_TOKEN_LIFETIME;
        let refresh_expires_at = now + constants::REFRESH_TOKEN_LIFETIME;

        /*let session = Session {
            id: 0,
            user: user,
            access_token: generate_token(),
            access_token_expires_at: access_expires_at,
            refresh_token: generate_token(),
            refresh_token_expires_at: refresh_expires_at,
            created_at: now,
            creation_ip_address: ip_address,
            creation_user_agent: user_agent
        };
        
        let item = SessionRecord::from(session);
        */

        let record = SessionRecord {
            id: 0, // to be replaced
            user_id: user_id,
            access_token: generate_token(),
            access_token_expires_at: access_expires_at,
            refresh_token: generate_token(),
            refresh_token_expires_at: refresh_expires_at,
            created_at: now,
            creation_ip_address: ip_address,
            creation_user_agent: user_agent
        };

        match self.repository.create(record).await {
            Ok(id) => {
                // TODO: log
                Ok(id)
            },
            Err(e) => {
                // TODO: log
                Err(format!("Failed to create user Sessoon. {}", e))
            }
        }
    }

    //pub async fn update(&self, item: Session) -> Result<(), String> {
    //    self.repository.update(item).await
    //}
}