use axum::response::{IntoResponseParts, ResponseParts};
use http::StatusCode;

#[derive(Debug)]
pub struct CookieResponse {}

impl IntoResponseParts for CookieResponse {
    type Error = StatusCode;

    fn into_response_parts(self, res: ResponseParts) -> Result<ResponseParts, Self::Error> {
        todo!()
    }
}
