//! # Cookie Authentication
//!
//! # Overview
//! - Cookie authentication is a method of authentication that involves using cookies to store user credentials.

use crate::{
    AuthnContext,
    authentication::handler::{Handler, SignInHandler, SignOutHandler},
};
mod cookies;
pub use cookies::*;

#[derive(Debug)]
pub enum CookieError {}

#[derive(Debug, Clone, new)]
pub struct CookieOption {}

impl Default for CookieOption {
    fn default() -> Self {
        Self {}
    }
}

#[derive(Debug, Clone)]
pub struct CookieState {}

// Cookie must be easy to closen we could use the inner scope
#[derive(Debug, Clone, new)]
pub struct CookieHandler {
    cookie_option: CookieOption,
}

pub trait CookieHandlerExt {
    type Error;
    fn sign_in_with_cookie<P>(&self, payload: P) -> impl Future<Output = Result<(), Self::Error>>
    where
        P: TryInto<cookies::Cookies>;
}

impl<T> CookieHandlerExt for T
where
    T: AuthnContext,
{
    type Error = ();

    // Complete this trait just a template for now
    async fn sign_in_with_cookie<P>(&self, payload: P) -> Result<(), Self::Error> {
        todo!()
    }
}

impl Handler for CookieHandler {
    type Error = CookieError;
    type State = CookieState;
    type Option = CookieOption;

    const NAME: &'static str = "Cookie";

    async fn authenticate(&self) -> Result<(), Self::Error> {
        todo!()
    }

    async fn forbid(&self, state: Self::State) -> Result<(), Self::Error> {
        todo!()
    }

    async fn challenge(&self, state: Self::State) -> Result<(), Self::Error> {
        todo!()
    }
}

impl SignOutHandler for CookieHandler {
    async fn sign_out(&self, state: Self::State) -> Result<(), Self::Error> {
        todo!()
    }
}

impl SignInHandler for CookieHandler {
    // change this to something cookie handler needed to sign in one of it something that could turn to vec of cookies
    type Payload = ();

    async fn sign_in(&self, state: Self::State, payload: Self::Payload) -> Result<(), Self::Error> {
        trace!("Got {:?} - {:?}", state, payload);
        trace!("Done authenticating....");

        Ok(())
    }
}
