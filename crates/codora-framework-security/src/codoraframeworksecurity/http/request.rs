use http::Request;

pub trait IntoCfSecurityRequest {
    type Body;

    fn into_cf_security_request(&self) -> Request<Self::Body>;
}
