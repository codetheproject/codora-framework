use crate::codoraframeworksecurity::http::IntoCfSecurityRequest;
use http::{Request, request::Parts};

impl IntoCfSecurityRequest for Parts {
    type Body = ();

    fn into_cf_security_request(&self) -> Request<Self::Body> {
        todo!()
    }
}
