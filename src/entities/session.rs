use chrono::{DateTime, Utc};

use crate::{entities::user::User, repositories::schemas::session_record::SessionRecord};

//#[derive(Serialize, Deserialize)]
//#[derive(Copy)]
#[derive(Clone)]
pub struct Session {
    pub id: i32,
    pub user: User,
    pub access_token: String,
    pub access_token_expires_at: DateTime<Utc>,
    pub refresh_token: String,
    pub refresh_token_expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub creation_ip_address: String,
    pub creation_user_agent: String,
}

impl From<(SessionRecord, User)> for Session {
    fn from((record, user): (SessionRecord, User)) -> Self {
        Session {
            id: record.id,
            user: user,        
            access_token: record.access_token,
            access_token_expires_at: record.access_token_expires_at,
            refresh_token: record.refresh_token,
            refresh_token_expires_at: record.refresh_token_expires_at,
            created_at: record.created_at,
            creation_ip_address: record.creation_ip_address,
            creation_user_agent: record.creation_user_agent
        }
    }
}

impl Session {

    /*
    pub fn is_access_token_active(&self) -> bool {
        self.access_token_expires_at > now()
    }

    pub fn is_refresh_token_active(&self) -> bool {
        self.refresh_token_expires_at > now()
    }
    */
}
