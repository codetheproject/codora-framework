use std::future::Ready;

use pin_project_lite::pin_project;
use valar_core::{
    http::IntoRequest,
    valar::handler::{Handler, State},
};

use crate::cookie::{CookieError, CookieResponse};

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

impl<Request> Handler<Request> for CookieHandler
where
    Request: IntoRequest + Sync + Send,
{
    type Response = CookieResponse;
    type Error = CookieError;
    type Future = Ready<Result<Self::Response, Self::Error>>;

    fn authenticate(&self, request: Request, state: &dyn State) -> Self::Future {
        todo!()
    }

    fn forbid(&self, request: Request, state: &dyn State) -> Self::Future {
        todo!()
    }

    fn challenge(&self, request: Request, state: &dyn State) -> Self::Future {
        todo!()
    }

    fn sign_out(&self, request: Request, state: &dyn State) -> Self::Future {
        todo!()
    }

    fn sign_in(&self, request: Request, state: &dyn State, credential: valar_core::valar::credential::Credential) -> Self::Future {
        todo!()
    }
}
