//! # Cookie Authentication
//!
//! # Overview
//! - Cookie authentication is a method of authentication that involves using cookies to store user credentials.

mod handler;
mod option;
mod payload;
mod response;
mod state;

pub use handler::CookieHandler;
pub use option::CookieOption;
pub use payload::CookiePayload;
pub use response::CookieResponse;
pub use state::CookieState;

use crate::Context;

#[derive(Debug)]
pub enum CookieError {}

/// Cookie Result
pub type Result<T = CookieResponse, E = CookieError> = std::result::Result<T, E>;

pub trait CookieHandlerExt {
    fn sign_in_with_cookie<S, P>(&self, state: S, payload: P) -> impl Future<Output = Result>
    where
        S: Into<CookieState>,
        P: Into<CookiePayload>;
}

impl<Request> CookieHandlerExt for Context<Request>
where
    Request: Sync,
{
    async fn sign_in_with_cookie<S, P>(&self, state: S, payload: P) -> Result
    where
        S: Into<CookieState>,
        P: Into<CookiePayload>,
    {
        self.sign_in::<CookieHandler>(state.into(), payload.into())
            .await
    }
}

#[cfg(test)]
mod tests {}
