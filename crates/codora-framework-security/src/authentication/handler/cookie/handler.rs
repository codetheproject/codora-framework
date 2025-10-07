#![allow(dead_code, unused_variables)]
use crate::{
    authentication::handler::{
        Handler, SignInHandler, SignOutHandler,
        cookie::{error::CookieError, option::CookieOption, payload::CookiePayload, response::CookieResponse, state::CookieState},
    },
    context::Context,
};
use std::sync::Arc;

// Cookie must be easy to clone we could use the inner scope
#[derive(Clone, new)]
pub struct CookieHandler {
    inner: Arc<InnerCookieHandler>,
}

#[derive(new)]
struct InnerCookieHandler {
    cookie_option: CookieOption,
}

#[cfg(feature = "axum")]
const _: () = {
    use http::request::Parts;

    impl Handler<Parts> for CookieHandler {
        type Error = CookieError;
        type State = CookieState;
        type Response = CookieResponse;

        const NAME: &'static str = "Cookie";

        async fn authenticate(&self, request: &Context<Parts>, state: Self::State) -> Result<Self::Response, Self::Error> {
            Ok(CookieResponse {})
        }

        async fn forbid(&self, request: &Context<Parts>, state: Self::State) -> Result<Self::Response, Self::Error> {
            Ok(CookieResponse {})
        }

        async fn challenge(&self, request: &Context<Parts>, state: Self::State) -> Result<Self::Response, Self::Error> {
            Ok(CookieResponse {})
        }
    }

    impl SignOutHandler<Parts> for CookieHandler {
        async fn sign_out(&self, request: &Context<Parts>, _state: Self::State) -> Result<Self::Response, Self::Error> {
            Ok(CookieResponse {})
        }
    }

    impl SignInHandler<Parts> for CookieHandler {
        // change this to something cookie handler needed to sign in one of it something that could turn to vec of cookies
        type Payload = CookiePayload;

        async fn sign_in(
            &self, request: &Context<Parts>, state: Self::State, payload: Self::Payload,
        ) -> Result<Self::Response, Self::Error> {
            Ok(CookieResponse {})
        }
    }
};
