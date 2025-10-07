//! # Cookie Authentication
//!
//! # Overview
//! - Cookie authentication is a method of authentication that involves using cookies to store user credentials.

use crate::codoraframeworksecurity::CFrameworkSecurity;

mod error;
mod handler;
mod option;
mod payload;
mod response;
mod state;

pub use error::CookieError;
pub use handler::CookieHandler;
pub use option::CookieOption;
pub use payload::CookiePayload;
pub use response::CookieResponse;
pub use state::CookieState;

// Cookie extension

pub trait CookieHandlerExt<Argument> {
    fn add_cookie(self, argument: Argument) -> Self;
}

impl<Argument> CookieHandlerExt<Argument> for CFrameworkSecurity
where
    Argument: Fn(CookieOption) -> CookieOption + Send + Sync + 'static,
{
    fn add_cookie(self, argument: Argument) -> Self {
        self.register_handler(CookieHandler::new(argument(CookieOption::default())))
    }
}

impl CookieHandlerExt<CookieOption> for CFrameworkSecurity {
    fn add_cookie(self, argument: CookieOption) -> Self {
        self.register_handler(CookieHandler::new(argument))
    }
}
