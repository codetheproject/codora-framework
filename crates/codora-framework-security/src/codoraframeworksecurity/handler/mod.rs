pub mod ext;

use super::{
    http::{IntoCfSecurityRequest, IntoCfSecurityResponse},
};



pub trait Handler<Request, State>
where
    Request: IntoCfSecurityRequest,
{
    type Response: IntoCfSecurityResponse;
    type Error: IntoCfSecurityResponse;

    /// The name of the handler
    /// This is used to identify the handler in logs and other contexts
    const NAME: &'static str;

    /// Authenticate the current request
    ///
    /// This method is called to authenticate the current request
    #[rustfmt::skip]
    fn authenticate(&self, request: Request, state: State) -> impl Future<Output = Result<Self::Response, Self::Error>> + Send + Sync;

    /// Forbid the current request
    ///
    /// This method is called to forbid the current request
    ///
    /// # Arguments
    /// `state` - This is used to pass current state to the handler could be derived from anywhere `S`
    #[rustfmt::skip]
    fn forbid(&self, request: Request, state: State) -> impl Future<Output = Result<Self::Response, Self::Error>> + Send + Sync;

    /// Challenge the current request
    ///
    /// This method is called to challenge the current request
    ///
    /// # Arguments
    /// `state` - The current state of the request `S`
    #[rustfmt::skip]
    fn challenge(&self, request: Request, state: State) -> impl Future<Output = Result<Self::Response, Self::Error>> + Send + Sync;
}

#[rustfmt::skip]
pub trait SignOutHandler<Request, State>: Handler<Request, State> 
where
    Request: IntoCfSecurityRequest,
{
    fn sign_out(&self, request: Request, state: State) -> impl Future<Output = Result<Self::Response, Self::Error>> + Send + Sync;
}

#[rustfmt::skip]
pub trait SignInHandler<Request, State, Payload>: SignOutHandler<Request, State> 
where
    Request: IntoCfSecurityRequest,
{

    fn sign_in(&self, request: Request, state: State, payload: Payload) -> impl Future<Output = Result<Self::Response, Self::Error>> + Send + Sync;
}
