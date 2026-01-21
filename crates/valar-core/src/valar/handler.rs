use crate::{
    BoxFuture,
    http::{IntoRequest, IntoResponse},
    valar::credential::Credential,
};
use std::any::Any;
use std::future::Future;

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

pub trait Handler<Request>
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

    fn sign_out(&self, request: Request, state: &dyn State) -> Self::Future;

    fn sign_in(&self, request: Request, state: &dyn State, credential: Credential) -> Self::Future;
}

// #[derive(Debug, Clone)]
// pub struct Handler<H> {
//     // Behaves like it own handler
//     _t: PhantomData<H>,

//     // This is the service that would be used to handle the request basically contain H
//     service: BoxCloneSyncService<Request, Response, Infallible>,
// }

// #[derive(Debug, Clone)]
// // pub struct HandlerAsService();

#[rustfmt::skip]
pub struct BoxCloneSyncHandler<T, U, E>(
    Box<
        dyn CloneHandler<T, Response = U, Error = E, Future = BoxFuture<'static, Result<U, E>>> 
            + Send
            + Sync
    >,
);

impl<T, U, E> BoxCloneSyncHandler<T, U, E>
where
    T: IntoRequest,
{
    pub fn new<H>(handler: H) -> Self
    where
        H: Handler<T, Response = U, Error = E> + Send + Sync + 'static,
        H::Future: Send + Sync + 'static,
    {
        // BoxCloneSyncHandler(Box::new(HandlerWrapper { handler }))

        todo!()
    }
}

impl<T, U, E> Handler<T> for BoxCloneSyncHandler<T, U, E>
where
    T: IntoRequest,
    U: IntoResponse,
    E: IntoResponse,
{
    type Response = U;
    type Error = E;
    type Future = BoxFuture<'static, Result<U, E>>;

    // This will forward the call to the inner handler
    fn authenticate(&self, request: T, state: &dyn State) -> Self::Future {
        self.0.authenticate(request, state)
    }
    fn forbid(&self, request: T, state: &dyn State) -> Self::Future {
        self.0.forbid(request, state)
    }
    fn challenge(&self, request: T, state: &dyn State) -> Self::Future {
        self.0.challenge(request, state)
    }
    fn sign_out(&self, request: T, state: &dyn State) -> Self::Future {
        self.0.sign_out(request, state)
    }
    fn sign_in(&self, request: T, state: &dyn State, credential: Credential) -> Self::Future {
        self.0
            .sign_in(request, state, credential)
    }
}

impl<T, U, E> std::fmt::Debug for BoxCloneSyncHandler<T, U, E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BoxCloneSyncHandler")
            .finish()
    }
}

impl<T, U, E> Clone for BoxCloneSyncHandler<T, U, E>
where
    T: IntoRequest,
    U: IntoResponse,
    E: IntoResponse,
{
    fn clone(&self) -> Self {
        Self(self.0.clone_box())
    }
}

#[rustfmt::skip]
pub trait CloneHandler<Request>: Handler<Request> 
where
    Request: IntoRequest,
{
    fn clone_box(&self) -> Box<
        dyn CloneHandler<Request, Response = Self::Response, Error = Self::Error, Future = Self::Future>
            + Send
            + Sync>
    ;
}

impl<T, R> CloneHandler<R> for T
where
    T: Handler<R> + Clone + 'static + Send + Sync,
    R: IntoRequest,
{
    fn clone_box(&self) -> Box<dyn CloneHandler<R, Response = T::Response, Error = T::Error, Future = T::Future> + Send + Sync> {
        Box::new(self.clone())
    }
}
