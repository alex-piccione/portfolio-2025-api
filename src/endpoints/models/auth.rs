pub mod login {
    use serde::{Serialize, Deserialize};

    #[derive(Deserialize)]
    pub struct Request {
        pub username: String,
        pub password: String,
    }

    #[derive(Serialize)]
    pub struct Response {
        pub is_success: bool,
        pub error: Option<String>,
    }
}