#![allow(dead_code, unused_variables)]
use super::{CookieError, CookieOption, CookieResponse, CookieState};
use crate::{
    codoraframeworksecurity::{
        handler::{Handler, SignInHandler, SignOutHandler},
        http::IntoCfSecurityRequest,
    },
    handlers::cookie::CookiePayload,
};
use std::sync::Arc;

// Cookie must be easy to clone we could use the inner scope
#[derive(Clone)]
pub struct CookieHandler {
    inner: Arc<InnerCookieHandler>,
}

impl CookieHandler {
    pub fn new(cookie_option: CookieOption) -> Self {
        let inner = Arc::new(InnerCookieHandler::new(cookie_option));
        Self { inner }
    }
}

#[derive(new)]
struct InnerCookieHandler {
    cookie_option: CookieOption,
}

impl<Request, State> Handler<Request, State> for CookieHandler
where
    Request: IntoCfSecurityRequest + Sync + Send,
    State: Into<CookieState> + Send + Sync,
{
    type Response = CookieResponse;
    type Error = CookieError;

    const NAME: &'static str = "Cookie";

    async fn authenticate(&self, request: Request, state: State) -> Result<Self::Response, Self::Error> {
        let request = request.into_cf_security_request();

        // implement authentication logic here

        Ok(CookieResponse::default())
    }

    async fn forbid(&self, request: Request, state: State) -> Result<Self::Response, Self::Error> {
        let request = request.into_cf_security_request();

        // implement forbid logic here

        Ok(CookieResponse::default())
    }

    async fn challenge(&self, request: Request, state: State) -> Result<Self::Response, Self::Error> {
        let request = request.into_cf_security_request();

        // implement challenge logic here

        Ok(CookieResponse::default())
    }
}

impl<Request, State> SignOutHandler<Request, State> for CookieHandler
where
    Request: IntoCfSecurityRequest + Send + Sync,
    State: Into<CookieState> + Sync + Send,
{
    async fn sign_out(&self, request: Request, _state: State) -> Result<Self::Response, Self::Error> {
        let request = request.into_cf_security_request();

        // implement sign out  logic here

        Ok(CookieResponse::default())
    }
}

impl<Request, State, Payload> SignInHandler<Request, State, Payload> for CookieHandler
where
    Request: IntoCfSecurityRequest + Send + Sync,
    State: Into<CookieState> + Sync + Send,
    Payload: Into<CookiePayload> + Send + Sync,
{
    async fn sign_in(&self, request: Request, state: State, payload: Payload) -> Result<Self::Response, Self::Error> {
        let request = request.into_cf_security_request();

        // implement sign in  logic here

        Ok(CookieResponse::default())
    }
}
