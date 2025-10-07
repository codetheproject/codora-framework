use std::future::Ready;

use pin_project_lite::pin_project;
use valar_core::{
    http::IntoRequest,
    valar::handler::{AuthenticationHandler, SignInHandler, SignOutHandler},
};

use crate::cookie::{CookieError, CookieResponse, CookieState};

// Cookie must be easy to clone we could use the inner scope
#[derive(Clone, new)]
pub struct CookieHandler {
    // Replace this with cookies definently
    cookies: Vec<()>,
}

pin_project! {
    pub struct CookieFuture<F> {
        #[pin]
        inner: F,
    }
}

impl std::fmt::Debug for CookieHandler {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CookieHandler").finish()
    }
}

impl<Request> AuthenticationHandler<Request> for CookieHandler
where
    Request: IntoRequest + Sync + Send,
{
    type Response = CookieResponse;
    type Error = CookieError;
    type State = CookieState;
    type Future = Ready<Result<Self::Response, Self::Error>>;

    fn authenticate(&self, request: Request, state: Self::State) -> Self::Future {
        todo!()
    }

    fn forbid(&self, request: Request, state: Self::State) -> Self::Future {
        todo!()
    }

    fn challenge(&self, request: Request, state: Self::State) -> Self::Future {
        todo!()
    }
}

impl<Request> SignOutHandler<Request> for CookieHandler
where
    Request: IntoRequest + Send + Sync,
{
    fn sign_out(&self, request: Request, state: Self::State) -> Self::Future {
        todo!()
    }
}

impl<Request> SignInHandler<Request> for CookieHandler
where
    Request: IntoRequest + Send + Sync,
{
    // Let assume (): Cookie
    type Payload = Vec<()>;

    fn sign_in(&self, request: Request, state: Self::State, payload: Self::Payload) -> Self::Future {
        todo!()
    }
}
