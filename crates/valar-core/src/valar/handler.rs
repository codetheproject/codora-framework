use crate::{
    http::{IntoRequest, IntoResponse, Request, Response},
    valar::credential::Credential,
};
use std::{any::Any, convert::Infallible, marker::PhantomData};
use tower::util::BoxCloneSyncService;

// TODO
// Handler should be able to created by default with default associated types for Response and Error
// This way user can use our api without much config and boilerplate

pub trait State: Any + Send + Sync {
    fn as_any(&self) -> &dyn Any;
}

// Blanket impl - any Send + Sync type is automatically a State
impl<T: Any + Send + Sync + 'static> State for T {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub fn resolve_state<T: 'static>(state: &dyn State) -> Option<&T> {
    state.as_any().downcast_ref::<T>()
}

pub trait AuthenticationHandler<Request>
where
    Request: IntoRequest,
{
    type Response: IntoResponse;
    type Error: IntoResponse;
    type Future: Future<Output = Result<Self::Response, Self::Error>> + Send + Sync;

    /// Authenticate the current request
    ///
    /// This method is called to authenticate the current request

    fn authenticate(&self, request: Request, state: &dyn State) -> Self::Future;

    /// Forbid the current request
    ///
    /// This method is called to forbid the current request
    ///
    /// # Arguments
    /// `state` - This is used to pass current state to the handler could be derived from anywhere `S`

    fn forbid(&self, request: Request, state: &dyn State) -> Self::Future;

    /// Challenge the current request
    ///
    /// This method is called to challenge the current request
    ///
    /// # Arguments
    /// `state` - The current state of the request `S`

    fn challenge(&self, request: Request, state: &dyn State) -> Self::Future;
}

pub trait SignOutHandler<Request>: AuthenticationHandler<Request>
where
    Request: IntoRequest,
{
    fn sign_out(&self, request: Request, state: &dyn State) -> Self::Future;
}

pub trait SignInHandler<Request>: AuthenticationHandler<Request>
where
    Request: IntoRequest,
{
    fn sign_in(&self, request: Request, state: &dyn State, credential: Credential) -> Self::Future;
}

#[derive(Debug, Clone)]
pub struct Handler<H> {
    // Behaves like it own handler
    _t: PhantomData<H>,

    // This is the service that would be used to handle the request basically contain H
    service: BoxCloneSyncService<Request, Response, Infallible>,
}

// #[derive(Debug, Clone)]
// // pub struct HandlerAsService();
