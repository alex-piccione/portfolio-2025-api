use serde::{Serialize};

#[derive(Serialize)]
pub struct SuccessErrorResponse {
    is_success: bool,
    error: Option<String>
}

impl SuccessErrorResponse {
    pub fn success() -> Self {
        Self { is_success: true, error: None}
    }

    pub fn error(message:&str) -> Self {
        Self { is_success: false, error: Some(message.to_string())}
    }
}

#[derive(Serialize)]
pub struct DataResponse<T> {
    #[serde(flatten)]
    pub base: SuccessErrorResponse,
    pub data: Option<T>
}

impl<T> DataResponse<T> {
    pub fn success(data:T) -> Self {
        DataResponse {
            base: SuccessErrorResponse::success(),
            data: Some(data)
        }
    }

    pub fn error(message:&str) -> Self {
        DataResponse { base: SuccessErrorResponse::error(message), data: None }
    }
}
