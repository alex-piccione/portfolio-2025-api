use crate::{entities::currency::Currency, utils::datetime::UtcDateTime};

pub struct User {
    pub id: String,
    pub username: String,
    pub hashed_password: String,
    pub creation_date: UtcDateTime,
    pub currency: Currency,
    pub role: String,
}

impl User {
    pub fn is_admin(&self) -> bool { self.role == "Admin" }
}