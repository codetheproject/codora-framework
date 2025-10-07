use crate::codoraframeworksecurity::http::{IntoCfSecurityResponse, Response};

#[derive(Debug, Default)]
pub struct CookieResponse {}

impl IntoCfSecurityResponse for CookieResponse {
    fn into_cf_security_response(self) -> Response {
        todo!()
    }
}
