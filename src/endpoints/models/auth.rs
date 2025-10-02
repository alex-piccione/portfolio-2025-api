// TODO: use a common structure
use serde::{Serialize};

#[derive(Serialize)]
pub struct OkErrorResponse {
    pub is_success: bool,
    pub error: Option<String>,
}

pub mod signup {
    #[derive(serde::Deserialize)]
    pub struct Request {
        pub username: String,
        pub password: String,
        pub currency_id: i32
    }

   /* #[derive(Serialize)]
    pub struct Response {
        pub is_success: bool,
        pub error: Option<String>,
    }*/
}

pub mod login {

    #[derive(serde::Deserialize)]
    pub struct Request {
        pub username: String,
        pub password: String,
    }

    /*#[derive(Serialize)]
    pub struct Response {
        pub is_success: bool,
        pub error: Option<String>,
    }*/
}