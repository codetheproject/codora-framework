use std::future::Future;

pub trait Handler {
    type Error: Send + Sync + 'static;
    type Option: Default + Send + Sync + 'static;
    type State: Send + Sync + 'static;

    /// The name of the handler
    /// This is used to identify the handler in logs and other contexts
    const NAME: &'static str;

    /// Authenticate the current request
    ///
    /// This method is called to authenticate the current request
    fn authenticate(&self) -> impl Future<Output = Result<(), Self::Error>> + Send + Sync;

    /// Forbid the current request
    ///
    /// This method is called to forbid the current request
    ///
    /// # Arguments
    /// `state` - This is used to pass current state to the handler could be derived from anywhere `S`
    fn forbid(&self, state: Self::State) -> impl Future<Output = Result<(), Self::Error>> + Send + Sync;

    /// Challenge the current request
    ///
    /// This method is called to challenge the current request
    ///
    /// # Arguments
    /// `state` - The current state of the request `S`
    fn challenge(&self, state: Self::State) -> impl Future<Output = Result<(), Self::Error>> + Send + Sync;
}

pub trait SignOutHandler: Handler {
    fn sign_out(&self, state: Self::State) -> impl Future<Output = Result<(), Self::Error>> + Send + Sync;
}

pub trait SignInHandler: SignOutHandler {
    type Payload: Send + Sync;

    // () should be a response for now let just assume it is
    fn sign_in(&self, state: Self::State, payload: Self::Payload) -> impl Future<Output = Result<(), Self::Error>> + Send + Sync;
}
