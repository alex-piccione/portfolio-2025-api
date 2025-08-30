// Axum handles the conversion of a simple string to the HTTP response
pub async fn home() -> &'static str {
    "Hello, Axum API (learning.Rust)!"
}
