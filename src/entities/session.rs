use chrono::{DateTime, Utc};

use crate::{entities::user::User};

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
