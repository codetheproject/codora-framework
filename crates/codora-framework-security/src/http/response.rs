// pub use http::Response;

pub type Response = http::Response<()>;

/// Explain IntoResponse how it works and the purpose of it
pub trait IntoResponse {
    fn into_response(self) -> Response;
}

// Implement IntoResponse for common types
impl IntoResponse for String {
    fn into_response(self) -> Response {
        todo!()
    }
}
