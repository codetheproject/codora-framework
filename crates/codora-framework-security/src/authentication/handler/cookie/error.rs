use crate::http::IntoResponse;

#[derive(Debug)]
pub enum CookieError {}

impl IntoResponse for CookieError {
    fn into_response(self) -> crate::http::Response {
        todo!()
    }
}
