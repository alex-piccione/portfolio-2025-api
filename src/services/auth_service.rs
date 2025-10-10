use crate::{ entities::{currency::Currency, session::Session, user::User}, repositories::{session_repository::SessionRepository}, services::{password_hashing::{hash_password, verify_password}, 
session_service::{SessionService}, user_service::{CreateError, UserService}}, utils::datetime::now};

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
    InvalidToken,
    ExpiredToken,
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
        
        //let user = self.user_service.find_by_username(request.username).await else {
        //    return login::Response::error()
        //};

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

    pub async fn validate_access_token(&self, access_token: &str) -> Result<Session, AuthError> {
        
        let Some(session) = 
            self.session_service.find_by_access_token(access_token).await
            .map_err(| e| AuthError::DatabaseError(e))? else {
                return Err(AuthError::InvalidToken); 
            };

        /* let Some(session_record) = 
            self.session_repository.find_by_access_token(access_token).await
            .map_err(| e| AuthError::DatabaseError(e))? else {
                return Err(AuthError::InvalidToken); 
            };*/

        if now() > session.access_token_expires_at {
            return Err(AuthError::ExpiredToken);
        };

        Ok(session)
    }

}

pub struct LoginRequest {
    pub username: String,
    pub password: String,
    pub ip_address: String,
    pub user_agent:String
}
