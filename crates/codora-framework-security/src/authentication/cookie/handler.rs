use crate::{
    Context,
    authentication::handler::{Handler, SignInHandler, SignOutHandler},
    cookie::{CookieError, CookieOption, CookiePayload, CookieResponse, CookieState},
};

use super::Result;

// Cookie must be easy to clone we could use the inner scope
#[derive(Debug, Clone, new)]
pub struct CookieHandler {
    cookie_option: CookieOption,
}

impl<Request> Handler<Request> for CookieHandler
where
    Request: Sync,
{
    type Error = CookieError;
    type State = CookieState;
    type Option = CookieOption;
    type Response = CookieResponse;

    const NAME: &'static str = "Cookie";

    async fn authenticate(&self, ctx: &Context<Request>) -> Result {
        todo!()
    }

    async fn forbid(&self, ctx: &Context<Request>, _state: Self::State) -> Result {
        todo!()
    }

    async fn challenge(&self, ctx: &Context<Request>, _state: Self::State) -> Result {
        todo!()
    }
}

impl<Request> SignOutHandler<Request> for CookieHandler
where
    Request: Sync,
{
    async fn sign_out(&self, ctx: &Context<Request>, _state: Self::State) -> Result {
        todo!()
    }
}

impl<Request> SignInHandler<Request> for CookieHandler
where
    Request: Sync,
{
    // change this to something cookie handler needed to sign in one of it something that could turn to vec of cookies
    type Payload = CookiePayload;

    async fn sign_in(&self, ctx: &Context<Request>, state: Self::State, payload: Self::Payload) -> Result {
        trace!("Got {:?} - {:?}", state, payload);
        trace!("Done authenticating....");

        todo!()
    }
}
