use body::Body;
use http::{Request as HTTPRequest, request::Parts};

pub mod body;

pub type Response<B = Body> = http::response::Response<B>;
pub type Request<B = Body> = http::request::Request<B>;

/// Explain IntoResponse how it works and the purpose of it
pub trait IntoResponse {
    type Body;

    fn into_response(self) -> Response<Self::Body>;
}

pub trait IntoRequest {
    type Body;

    fn into_request(self) -> Request<Self::Body>;
}

impl IntoRequest for Parts {
    type Body = ();

    fn into_request(self) -> Request<Self::Body> {
        HTTPRequest::from_parts(self, ())
    }
}

impl IntoRequest for Request<Body> {
    type Body = Self;

    fn into_request(self) -> Request<Self::Body> {
        todo!()
    }
}

impl IntoResponse for Response<Body> {
    type Body = Self;

    fn into_response(self) -> Response<Self::Body> {
        todo!()
    }
}

#[cfg(test)]
mod tests {

    #[tokio::test]
    async fn check_http() -> anyhow::Result<()> {
        Ok(())
    }
}
