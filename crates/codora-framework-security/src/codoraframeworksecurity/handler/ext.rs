//! Handler extension method's
use crate::codoraframeworksecurity::Error;

pub trait HandlerExt<State> {
    type Response;

    fn authenticate(self, state: State) -> impl Future<Output = Result<Self::Response, Error>>;
    fn challenge(self, state: State) -> impl Future<Output = Result<Self::Response, Error>>;
    fn forbid(self, state: State) -> impl Future<Output = Result<Self::Response, Error>>;
}

pub trait SignOutHandlerExt<State>: HandlerExt<State> {
    fn sign_out(self, state: State) -> impl Future<Output = Result<Self::Response, Error>>;
}

pub trait SignInHandlerExt<State, Payload>: SignOutHandlerExt<State> {
    fn sign_in(self, state: State, payload: Payload) -> impl Future<Output = Result<Self::Response, Error>>;
}
