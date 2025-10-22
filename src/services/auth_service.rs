use rand::RngCore;
use base64::{engine::general_purpose, Engine as _};

use crate::{ constants, entities::{currency::Currency, session::Session, user::User}, repositories::{schemas::session_record::{SessionRecord, SessionWithUser, UpdateForAccess, UpdateForRefresh}, session_repository::SessionRepository}, services::{password_hashing::{hash_password, verify_password}, 
session_service::SessionService, user_service::{CreateError, UserService}}, utils::datetime::{self, now}};

#[derive(Clone)]
pub struct AuthService {
    user_service: UserService,
    session_service: SessionService,
    session_repository: SessionRepository
}

pub enum LoginError {
    DatabaseError(String),
    FailedLogin
}

#[derive(Debug)]
pub enum AuthError {
    DatabaseError(String),
    InvalidOrExpiredToken(String),
}

pub fn generate_token() -> String {
    let mut bytes = [0u8; 48];
    rand::rng().fill_bytes(&mut bytes);
    general_purpose::URL_SAFE_NO_PAD.encode(bytes)
}

impl AuthService {
    pub fn new (user_service: UserService, session_service: SessionService, session_repository: SessionRepository) -> Self {
        AuthService { user_service, session_service, session_repository}
    }

    pub async fn signup(&self, username:String, password:String, currency:Currency) -> Result<(), CreateError> {

        let id = uuid::Uuid::new_v4().to_string();
        let hashed_password = hash_password(&password);

        let user:User = User {
            id: id,
            username: username,
            hashed_password: hashed_password,
            creation_date: now(),
            currency,
            role: String::from("User"), // default
        };

        self.user_service.create(user).await 
    }

    pub async fn login(&self, request:LoginRequest) -> Result<Session, LoginError> {

        let Some(user) =
            self.user_service.find_by_username(request.username).await 
                .map_err(|e| LoginError::DatabaseError(e))?
        else {
            return Err(LoginError::FailedLogin);
        };

        match verify_password(&request.password, &user.hashed_password) {
            true =>  {
                // create session
                self.session_service.create(user, request.ip_address, request.user_agent).await 
                    .map_err(|e| LoginError::DatabaseError(e))
            }, 
            false => Err(LoginError::FailedLogin)
        }
    }

    /// Prolonge the access and refresh token validity and return the session with user id
    pub async fn validate_access(&self, access_token: String) -> Result<SessionWithUser, AuthError> {
        let now = datetime::now();

        let data_for_expired_token = format!("update_for_access: 
        access_token: {},
        now: {},
        access_token_expires_at: {},
        refresh_token_expires_at: {},
        ", 
        access_token,
        now,
        now + constants::auth::ACCESS_TOKEN_LIFETIME,
        now + constants::auth::REFRESH_TOKEN_LIFETIME);

        match self.session_repository.update_for_access(UpdateForAccess {
            access_token,
            access_token_expires_at: now + constants::auth::ACCESS_TOKEN_LIFETIME,
            refresh_token_expires_at: now + constants::auth::REFRESH_TOKEN_LIFETIME,
            last_access_at: now
        }).await
            .map_err(|e| AuthError::DatabaseError(e))? {
                Some(record) => Ok(record),
                None => Err(AuthError::InvalidOrExpiredToken(data_for_expired_token)) // session not found
        }
    }

    pub async fn refresh_session(&self, refresh_token: String) -> Result<SessionRecord, AuthError> {
        let now = datetime::now();
        
        // debug
        let session = 
            match self.session_repository.find_by_refresh_token(&refresh_token).await {
                Err(_) => None,
                Ok(record) => record
            };

        let (session_id, refresh_token_expires_at) = match session {
            Some(s) => (s.id.to_string(), s.refresh_token_expires_at.to_string()),
            None => ("".to_string(), "".to_string())
        };

        let data_for_expired_token = format!("refresh_session.
            refresh_token: {}, 
            now: {},
            access_token_expires_at: {},
            refresh_token_expires_at: {},
            session: {},
            session.refresh_token_expires_at: {}
            ", 
            refresh_token,
            now,
            now + constants::auth::ACCESS_TOKEN_LIFETIME,
            now + constants::auth::REFRESH_TOKEN_LIFETIME,
            session_id,
            refresh_token_expires_at
        );
        
        match self.session_repository.update_for_refresh(UpdateForRefresh {
            old_refresh_token: refresh_token,
            access_token: generate_token(),
            refresh_token: generate_token(),
            access_token_expires_at: now + constants::auth::ACCESS_TOKEN_LIFETIME,
            refresh_token_expires_at: now + constants::auth::REFRESH_TOKEN_LIFETIME,
            last_refresh_at: now
        }).await
            .map_err(|e| AuthError::DatabaseError(e))? {
                Some(record) => Ok(record),
                None => Err(AuthError::InvalidOrExpiredToken(data_for_expired_token)) // session not found
        }
    }

}

pub struct LoginRequest {
    pub username: String,
    pub password: String,
    pub ip_address: String,
    pub user_agent:String
}
