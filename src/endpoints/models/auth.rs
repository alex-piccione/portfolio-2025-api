
pub mod signup {
    use crate::endpoints::models::common;

    #[derive(serde::Deserialize)]
    pub struct Request {
        pub username: String,
        pub password: String,
        pub currency_id: i32
    }

    pub type Response = common::SuccessErrorResponse;
}

pub mod login {
    use crate::{endpoints::models::common::DataResponse, utils::datetime::UtcDateTime};

    #[derive(serde::Deserialize)]
    pub struct Request {
        pub username: String,
        pub password: String,
    }

    pub type Response = DataResponse<Session>;

    #[derive(serde::Serialize)]
    pub struct Session {
        pub access_token: String,
        pub access_token_expires_at: UtcDateTime,
        pub refresh_token: String,
        pub refresh_token_expires_at: UtcDateTime
    }
}