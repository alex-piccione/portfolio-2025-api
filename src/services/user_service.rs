use crate::{
    entities::user::User, 
    repositories::user_repository::UserRepository, services::currency_provider::CurrencyProvider, utils::datetime::{UtcDateTime}};

#[derive(Clone)]
pub struct UserService {
    user_repository: UserRepository, 
    //currency_provider: CurrencyProvider
}

impl UserService {
    pub fn new(user_repository: UserRepository) -> Self {
        Self { user_repository}
    }

    pub async fn create(&self, user:User) -> Result<(), String> {
        self.user_repository.create(user).await
    }

    pub async fn try_get_by_username(&self, username: String) -> Result<Option<User>, String> {
        match &self.user_repository.try_get_by_username(username).await {
            Ok(option) => {
                match option {
                    Some(record) => {
                        let user = User {
                            id: record.id.clone(),
                            username: record.username.clone(),
                            hashed_password: record.hashed_password.clone(),
                            creation_date: UtcDateTime::from_timestamptz(record.creation_date),
                            currency: CurrencyProvider::get(record.currency_id),
                            role: record.role.clone()
                        };

                        Ok(Some(user))
                    },
                    None => Ok(None)
                }
            },
            Err(e) => Err(format!("{}", e))
        }
    } 
}