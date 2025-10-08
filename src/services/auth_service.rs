use crate::{ entities::{currency::Currency, session::Session, user::User}, 
services::{password_hashing::{hash_password, verify_password}, 
session_service::SessionService, user_service::{CreateError, UserService}}, utils::datetime::now};


#[derive(Clone)]
pub struct AuthService {
    user_service: UserService,
    session_service: SessionService
}

pub enum LoginError {
    DatabaseError(String),
    FailedLogin
}

impl AuthService {
    pub fn new (user_service: UserService, session_service: SessionService) -> Self {
        AuthService { user_service, session_service}
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
/*
        let user = self.user_service.find_by_username(request.username).await else {
            return login::Response::error()
        };
        */

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
}

pub struct LoginRequest {
    pub username: String,
    pub password: String,
    pub ip_address: String,
    pub user_agent:String
}
