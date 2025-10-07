use crate::context::Context;

// Handler implementaion
mod bearer;
mod cookie;
mod jwt;
mod oauth;

// Expose cookie api
pub use cookie::{
    CookieHandlerExt, error::CookieError, handler::CookieHandler, option::CookieOption, payload::CookiePayload, response::CookieResponse,
    state::CookieState,
};

pub trait Handler<Request> {
    type Error;
    type Response;
    type State: Send + Sync + 'static;

    /// The name of the handler
    /// This is used to identify the handler in logs and other contexts
    const NAME: &'static str;

    /// Authenticate the current request
    ///
    /// This method is called to authenticate the current request
    #[rustfmt::skip]
    fn authenticate(&self, request: &Context<Request>, state: Self::State) -> impl Future<Output = Result<Self::Response, Self::Error>> + Send + Sync;

    /// Forbid the current request
    ///
    /// This method is called to forbid the current request
    ///
    /// # Arguments
    /// `state` - This is used to pass current state to the handler could be derived from anywhere `S`
    #[rustfmt::skip]
    fn forbid(&self, request: &Context<Request>, state: Self::State) -> impl Future<Output = Result<Self::Response, Self::Error>> + Send + Sync;

    /// Challenge the current request
    ///
    /// This method is called to challenge the current request
    ///
    /// # Arguments
    /// `state` - The current state of the request `S`
    #[rustfmt::skip]
    fn challenge(&self, request: &Context<Request>, state: Self::State) -> impl Future<Output = Result<Self::Response, Self::Error>> + Send + Sync;
}

#[rustfmt::skip]
pub trait SignOutHandler<Request>: Handler<Request> {
    fn sign_out(&self, request: &Context<Request>, state: Self::State) -> impl Future<Output = Result<Self::Response, Self::Error>> + Send + Sync;
}

#[rustfmt::skip]
pub trait SignInHandler<Request>: SignOutHandler<Request> {
    type Payload: Send + Sync;

    fn sign_in(&self, request: &Context<Request>, state: Self::State, payload: Self::Payload) -> impl Future<Output = Result<Self::Response, Self::Error>> + Send + Sync;
}
