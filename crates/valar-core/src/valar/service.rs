use crate::{
    http::{IntoRequest, IntoResponse},
    valar::{
        credential::Credential,
        handler::{Handler, State},
    },
};
use core::{future::Future, result::Result};

#[derive(Clone)]
pub struct HandlerService<H> {
    handler: H,
}

impl<H> HandlerService<H> {
    pub fn new(handler: H) -> Self {
        Self { handler }
    }
}

pub trait Service<Request> {
    type Response;
    type Error;
    type Future: Future<Output = Result<Self::Response, Self::Error>>;

    fn handle(&self, req: Request, state: &dyn State, credential: Option<Credential>) -> Self::Future;

    // We still wanna be sure that we wanna pass in state but let proceed
    fn authenticate(&self, req: Request, state: &dyn State) -> Self::Future {
        // Default implementation calls handle with Scope registered
        self.handle(req, state, None)
    }

    fn forbid(&self, req: Request, state: &dyn State) -> Self::Future {
        self.handle(req, state, None)
    }

    fn challenge(&self, req: Request, state: &dyn State) -> Self::Future {
        self.handle(req, state, None)
    }

    fn sign_out(&self, req: Request, state: &dyn State) -> Self::Future {
        self.handle(req, state, None)
    }

    fn sign_in(&self, req: Request, state: &dyn State, credential: Credential) -> Self::Future {
        self.handle(req, state, Some(credential))
    }
}

impl<Request, H> Handler<Request> for HandlerService<H>
where
    H: Service<Request>,
    Request: IntoRequest,
    H::Response: IntoResponse,
    H::Error: IntoResponse,
    H::Future: Send + Sync,
{
    type Response = H::Response;
    type Error = H::Error;
    type Future = H::Future;

    fn authenticate(&self, request: Request, state: &dyn State) -> Self::Future {
        self.handler
            .authenticate(request, state)
    }

    fn forbid(&self, request: Request, state: &dyn State) -> Self::Future {
        self.handler.forbid(request, state)
    }

    fn challenge(&self, request: Request, state: &dyn State) -> Self::Future {
        self.handler.challenge(request, state)
    }

    fn sign_out(&self, request: Request, state: &dyn State) -> Self::Future {
        self.handler.sign_out(request, state)
    }

    fn sign_in(&self, request: Request, state: &dyn State, credential: Credential) -> Self::Future {
        self.handler
            .sign_in(request, state, credential)
    }
}
