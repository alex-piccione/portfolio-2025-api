use std::time::Duration;

pub const ACCESS_TOKEN_LIFETIME: Duration = Duration::from_secs(30 * 60); // 30 minutes
pub const REFRESH_TOKEN_LIFETIME: Duration = Duration::from_secs(30 * 24 * 60 * 60); // 30 days