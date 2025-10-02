use crate::{entities::currency::Currency, utils::datetime::UtcDateTime};

pub struct User {
    pub id: String,
    pub username: String,
    pub hashed_password: String,
    pub creation_date: UtcDateTime,
    pub currency: Currency,
}