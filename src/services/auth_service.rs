use crate::{ entities::{currency::Currency, session::Session, user::User}, services::{password_hashing::{hash_password, verify_password}, session_service::SessionService, user_service::{CreateError, UserService}}, utils::datetime::{now, UtcDateTime}};


#[derive(Clone)]
pub struct AuthService {
    user_service: UserService,
    session_service: SessionService
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

    pub async fn login(&self, request:LoginRequest) -> Result<Session, String> {

        match self.user_service.find_by_username(request.username).await {
            Ok(option) => {
                match option {
                    Some(user) => {
                        match verify_password(&request.password, &user.hashed_password) {
                            true =>  {
                                // create session
                                match self.session_service.create(user, request.ip_address, request.user_agent).await {
                                    Ok(session) => {
                                        Ok(session)
                                    },
                                    Err(e) => {
                                        // log
                                        Err(format!("Some error occurred. Please retry."))
                                    }
                                }                                
                            }, 
                            false => Err(format!("Username or password are wrong"))
                        }                    
                    },
                    None => Err(format!("Username or password are wrong"))
                }
            },
            Err(e) => {
                // log
                Err(format!("Some error occurred. Please retry."))
            }
        }  
    }
}

pub struct LoginRequest {
    pub username: String,
    pub password: String,
    pub ip_address: String,
    pub user_agent:String
}

pub struct LoginResponse {
    pub is_success: bool,
    pub session: Option<Session>
    //pub access_token: String,
    //pub access_token_expires_at: UtcDateTime,
    //pub refresh_token: String,
    //pub refresh_token_expires_at: UtcDateTime
}