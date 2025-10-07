use crate::codoraframeworksecurity::http::IntoCfSecurityResponse;

#[derive(Debug)]
pub enum CookieError {}

impl IntoCfSecurityResponse for CookieError {
    fn into_cf_security_response(self) -> crate::codoraframeworksecurity::http::Response {
        todo!()
    }
}
