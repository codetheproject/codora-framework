use crate::{
    http::{IntoRequest, IntoResponse, Response},
    valar::{
        Valar,
        credential::Credential,
        handler::{Handler, State},
    },
};

pub trait AuthenticationContext<Request>: Sized
where
    Request: IntoRequest,
{
    fn auth(&self, request: Request) -> Context<'_, Request>;
}

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
    pub async fn authenticate<H>(self, state: &dyn State) -> Result<Response, Error>
    where
        H: Handler<Request> + Clone + Send + Sync + 'static,
        H::Response: IntoResponse,
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
    pub async fn forbid<H>(self, state: &dyn State) -> Result<Response, Error>
    where
        H: Handler<Request>,
    {
        todo!()
    }
    pub async fn challenge<H>(self, state: &dyn State) -> Result<Response, Error>
    where
        H: Handler<Request>,
    {
        todo!()
    }

    pub async fn sign_out<H>(self, state: &dyn State) -> Result<Response, Error>
    where
        H: Handler<Request>,
    {
        todo!()
    }

    pub async fn sign_in<H>(self, state: &dyn State, credential: Credential) -> Result<Response, Error>
    where
        H: Handler<Request>,
    {
        todo!()
    }
}

#[cfg(test)]
mod tests {}
