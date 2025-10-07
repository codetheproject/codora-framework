use std::future::Future;

use crate::Context;

pub trait Handler<Request> {
    type Error: Send + Sync + 'static;
    type State: Send + Sync + 'static;
    type Response: Send + Sync + 'static;
    type Option: Default + Send + Sync + 'static;

    /// The name of the handler
    /// This is used to identify the handler in logs and other contexts
    const NAME: &'static str;

    /// Authenticate the current request
    ///
    /// This method is called to authenticate the current request
    fn authenticate(&self, ctx: &Context<Request>) -> impl Future<Output = Result<Self::Response, Self::Error>> + Send + Sync;

    /// Forbid the current request
    ///
    /// This method is called to forbid the current request
    ///
    /// # Arguments
    /// `state` - This is used to pass current state to the handler could be derived from anywhere `S`
    fn forbid(&self, ctx: &Context<Request>, state: Self::State)
    -> impl Future<Output = Result<Self::Response, Self::Error>> + Send + Sync;

    /// Challenge the current request
    ///
    /// This method is called to challenge the current request
    ///
    /// # Arguments
    /// `state` - The current state of the request `S`
    fn challenge(
        &self, ctx: &Context<Request>, state: Self::State,
    ) -> impl Future<Output = Result<Self::Response, Self::Error>> + Send + Sync;
}

#[rustfmt::skip]
pub trait SignOutHandler<Request>: Handler<Request> {
    fn sign_out(&self, ctx: &Context<Request>, state: Self::State) -> impl Future<Output = Result<Self::Response, Self::Error>> + Send + Sync;
}

#[rustfmt::skip]
pub trait SignInHandler<Request>: SignOutHandler<Request> {
    type Payload: Send + Sync;

    // () should be a response for now let just assume it is
    fn sign_in(&self, ctx: &Context<Request>, state: Self::State, payload: Self::Payload) -> impl Future<Output = Result<Self::Response, Self::Error>> + Send + Sync;
}
