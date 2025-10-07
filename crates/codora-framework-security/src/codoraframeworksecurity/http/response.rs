// Replace this with Real Response

#[derive(Debug)]
pub struct Response;

impl std::fmt::Display for Response {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

/// Explain IntoResponse how it works and the purpose of it
pub trait IntoCfSecurityResponse {
    fn into_cf_security_response(self) -> Response;
}

// Implement IntoResponse for common types

// this should only be enabled when
impl IntoCfSecurityResponse for String {
    fn into_cf_security_response(self) -> Response {
        todo!()
    }
}
