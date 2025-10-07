mod context;
mod handler;

// Handler implementaion
pub mod cookie;

// /** given an authentication method like username and password to create a session
//  * method --> ctx --> handler --> response
//  *
//  * method should produce what handler needed like cookie, jwt, token then handler issue a response
//  *
//  * for now we assumed we don't have method but we have context
//  * context should be agnositic which means each framework define it's own context
//  */
pub use context::{Context, builder::ContextBuilder, extension::ContextHandlerExtension};
pub use handler::{Handler, SignInHandler, SignOutHandler};
