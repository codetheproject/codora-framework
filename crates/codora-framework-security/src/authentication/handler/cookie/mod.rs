//! # Cookie Authentication
//!
//! # Overview
//! - Cookie authentication is a method of authentication that involves using cookies to store user credentials.

use crate::{
    authentication::handler::{
        CookieError, CookieHandler, CookieResponse, Handler, SignInHandler, SignOutHandler,
        cookie::{payload::CookiePayload, state::CookieState},
    },
    context::Context,
};

pub(crate) mod error;
pub(crate) mod handler;
pub(crate) mod option;
pub(crate) mod payload;
pub(crate) mod response;
pub(crate) mod state;

pub trait CookieHandlerExt {
    fn sign_in_with_cookie<S, P>(&self, state: S, payload: P) -> impl Future<Output = Result<CookieResponse, CookieError>>
    where
        S: Into<CookieState>,
        P: Into<CookiePayload>;

    fn sign_out_with_cookie<S, P>(&self, state: S) -> impl Future<Output = Result<CookieResponse, CookieError>>
    where
        S: Into<CookieState>,
        P: Into<CookiePayload>;
}

// These are helper method to make sign in and sign out easier
impl<Request> CookieHandlerExt for Context<Request>
where
    CookieHandler: Handler<Request, Response = CookieResponse, Error = CookieError, State = CookieState>
        + SignInHandler<Request, Payload = CookiePayload>
        + SignOutHandler<Request>,
{
    async fn sign_in_with_cookie<S, P>(&self, state: S, payload: P) -> Result<CookieResponse, CookieError>
    where
        S: Into<CookieState>,
        P: Into<CookiePayload>,
    {
        self.sign_in::<CookieHandler>(state.into(), payload.into())
            .await
    }

    async fn sign_out_with_cookie<S, P>(&self, state: S) -> Result<CookieResponse, CookieError>
    where
        S: Into<CookieState>,
        P: Into<CookiePayload>,
    {
        self.sign_out::<CookieHandler>(state.into())
            .await
    }
}
