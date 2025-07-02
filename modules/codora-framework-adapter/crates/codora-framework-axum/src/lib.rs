//! Document codora support for axum framework
//!
#![forbid(unsafe_code)]
// Silence the noise in development!
#![cfg_attr(debug_assertions, allow(dead_code, unused_variables, warnings))]
// Docs and linting rules
#![cfg_attr(docsrs, feature(doc_auto_cfg, doc_cfg))]
#![cfg_attr(test, allow(clippy::float_cmp))]
#![cfg_attr(not(test), deny(clippy::print_stdout, clippy::dbg_macro))]
// - Lint for missing docs
#![cfg_attr(not(debug_assertions), deny(missing_docs))]

#[macro_use]
extern crate derive_new;

#[macro_use]
extern crate tracing;

use axum::{
    extract::FromRequestParts,
    http::{Extensions, request::Parts},
};
use codora_framework_web::security::AuthnContext;
use http::StatusCode;

#[derive(Clone)]
pub struct Context {
    // This is cheap to clone in axum codebase so we could have without arc
    extension: Extensions,
}

impl Context {
    pub fn new() -> Self {
        let extension = Extensions::default();
        Self { extension }
    }
}

impl<S> FromRequestParts<S> for Context
where
    S: Send + Sync,
{
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        parts
            .extensions
            .get::<Context>()
            .cloned()
            // ofc change this to handle error perfectly but this is something we wanna do
            .ok_or(StatusCode::INTERNAL_SERVER_ERROR)
    }
}

impl AuthnContext for Context {
    fn register_extension<T>(&mut self, ext: T) -> &mut Self
    where
        T: Send + Sync + 'static + Clone,
    {
        self.extension.insert(ext);
        self
    }

    fn get_extension<T>(&self) -> Option<&T>
    where
        T: Send + Sync + 'static + Clone,
    {
        self.extension.get::<T>()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use axum::{Extension, Router, body::Body, routing::get};
    use codora_framework_web::security::{CookieError, CookieHandler, CookieHandlerExt, CookieOption, CookieState, Cookies};
    use http::Request;
    use tower::ServiceExt as _;
    use tracing_subscriber::EnvFilter;

    #[tokio::test]
    async fn test_context() -> anyhow::Result<()> {
        tracing_subscriber::fmt()
            .with_env_filter(EnvFilter::try_from_env("RUST_LOG").unwrap_or(EnvFilter::from("trace")))
            .init();

        async fn handler(ctx: Context) -> ((), &'static str) {
            #[derive(Debug, Clone)]
            struct GoogleCookie {}

            /// This is just an example and should be automatically derived from the macro Cookie
            /// ex:
            /// #[derive(Debug, Clone, Cookie)]
            /// struct GoogleCookie {
            /// }
            impl TryFrom<GoogleCookie> for Cookies {
                type Error = CookieError;
                fn try_from(_value: GoogleCookie) -> Result<Self, Self::Error> {
                    todo!()
                }
            }

            trace!("Got called !!!");
            // properties
            // handler + payload

            // This should be simplify further as cookie state should be easily derived either from extension or
            let _cookie_state = CookieState {};

            //  we use usnit here as payload but you get the idea this is how we wanna authenticate

            // hmmmmm: tryinto for now but going from T to Cookies could be infallible hence changing TryFrom<Cookies> to From<Cookies> we should look into that
            let cookies: Cookies = GoogleCookie {}.try_into().unwrap();
            if let Ok(res) = ctx.sign_in_with_cookie(cookies).await {
                return (res, "Hello welcome you've been authenticated!");
            }
            // we wanna do something like this but would be nice if we could have

            // pass properties and payload only no need of the generic handler
            // let response = ctx.sign_in_with_cookie().await?;
            ((), "Failed to authenticate user")
        }

        let auth_context = Context::new().configure(|_ctx| {
            //  we have access to ctx here we could do something like still thinking if this would be mutable or not
            trace!("Yes we bout to configure options here");
            CookieHandler::new(CookieOption::new())
        });

        let app: Router<()> = Router::new()
            .route("/", get(handler))
            // .layer(ContextLayer::new(auth_context));
            // let just use extension for now
            .layer(Extension(auth_context));

        let request = Request::builder()
            .uri("/")
            .method("GET")
            .body(Body::empty())?;

        app.oneshot(request).await?;
        Ok(())
    }
}
