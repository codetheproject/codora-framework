use tower::Service;

use crate::{
    http::{IntoRequest, IntoResponse, Response, body::Body},
    metrics::Metrics,
    valar::{
        Valar,
        credential::Credential,
        handler::{AuthenticationHandler, SignInHandler, SignOutHandler, State},
    },
};

#[derive(Debug, new)]
pub struct Context<'a, Request> {
    valar_instace: &'a Valar,
    request: Request,
}

#[derive(Debug, thiserror::Error)]
#[error("{0}")]
pub enum Error {
    #[error("Handler not found")]
    HandlerNotFound,
}

impl<'a, Request> Context<'a, Request>
where
    Request: IntoRequest,
{
    pub async fn authenticate<H, T>(self, state: T) -> Result<Response, Error>
    where
        H: AuthenticationHandler<Request> + Clone + Send + Sync + 'static,
        H::Response: IntoResponse,
        T: Sync + Send + 'static,
    {
        let handler = self
            .valar_instace
            .get_handler::<H>()
            .ok_or_else(|| Error::HandlerNotFound)?;

        // let request = Request::new(self.request.into_request(), None, Some(Box::new(state)));
        // handler.call(request).await
        //

        // match handler.call(self.request).await {
        //     Ok(response) => {
        //         self.valar_instace.record_metrics(());

        //         todo!()
        //         // Ok(response.into_response().map(Body::new))
        //     }
        //     Err(error) => {
        //         //

        //         todo!()
        //     }
        // }

        todo!()
    }
    pub async fn forbid<H, T>(self, state: T) -> Result<Response, Error>
    where
        H: AuthenticationHandler<Request>,
    {
        todo!()
    }
    pub async fn challenge<H, T>(self, state: T) -> Result<Response, Error>
    where
        H: AuthenticationHandler<Request>,
    {
        todo!()
    }

    pub async fn sign_out<H, T>(self, state: T) -> Result<Response, Error>
    where
        H: SignOutHandler<Request>,
    {
        todo!()
    }

    pub async fn sign_in<H, T>(self, state: T, credential: Credential) -> Result<Response, Error>
    where
        H: SignInHandler<Request>,
    {
        todo!()
    }
}

#[cfg(test)]
mod tests {}
