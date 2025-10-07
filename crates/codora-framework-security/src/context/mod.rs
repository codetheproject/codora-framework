use crate::authentication::handler::{Handler, SignInHandler, SignOutHandler};
pub use extensions::Extension;

pub(super) mod extensions;

#[derive(new)]
pub struct CFrameworkSecurity {
    extension: Extension,
}

impl CFrameworkSecurity {
    pub fn register_handler<H>(self, handler: H) -> Self
    where
        H: Clone + Sync + 'static,
    {
        drop(handler);
        todo!()
    }
}

impl Clone for CFrameworkSecurity {
    fn clone(&self) -> Self {
        Self {
            extension: self.extension.clone(),
        }
    }
}

#[cfg(feature = "axum")]
const _: () = {
    use crate::context::{Context, Extension};
    use axum::{extract::Request, response::Response};
    use pin_project_lite::pin_project;
    use std::{pin::Pin, task::Poll};
    use tower_service::Service;

    #[derive(new, Clone)]
    pub struct CFrameworkService<S> {
        inner: S,
        extension: Extension,
    }

    pin_project! {
        #[derive(Debug, new)]
        pub struct ServiceResponseFuture<F> {
            #[pin]
            future: F
        }
    }

    impl<S> Service<Request> for CFrameworkService<S>
    where
        S: Service<Request, Response = Response> + Send + 'static,
        S::Future: Send + 'static,
    {
        type Response = S::Response;
        type Error = S::Error;
        type Future = ServiceResponseFuture<S::Future>;

        fn poll_ready(&mut self, cx: &mut std::task::Context<'_>) -> Poll<Result<(), Self::Error>> {
            self.inner.poll_ready(cx)
        }

        fn call(&mut self, request: Request) -> Self::Future {
            // Populate Context and attach it to the request's extensions.
            let (mut parts, body) = request.into_parts();

            let prev = parts
                .extensions
                .insert(Context::new(parts.clone(), self.extension.clone()));
            debug_assert!(prev.is_none(), "Context already present in request extensions");

            // Rebuild the request with the injected extensions and forward it.
            let request = Request::from_parts(parts, body);
            ServiceResponseFuture::new(self.inner.call(request))
        }
    }

    impl<S> tower_layer::Layer<S> for CFrameworkSecurity {
        type Service = CFrameworkService<S>;

        fn layer(&self, inner: S) -> Self::Service {
            CFrameworkService::new(inner, self.extension.clone())
        }
    }

    impl<F> Future for ServiceResponseFuture<F>
    where
        F: Future,
    {
        type Output = F::Output;

        fn poll(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> Poll<Self::Output> {
            let this = self.project();

            // Forward the inner future's poll result directly.
            match this.future.poll(cx) {
                Poll::Ready(output) => Poll::Ready(output),
                Poll::Pending => Poll::Pending,
            }
        }
    }
};

impl std::fmt::Debug for CFrameworkSecurity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CFrameworkSecurity")
            .field("extension", &self.extension)
            .finish()
    }
}

impl Default for CFrameworkSecurity {
    fn default() -> Self {
        CFrameworkSecurity::new(Extension::default())
    }
}

#[derive(new, Clone)]
pub struct Context<Request> {
    request: Request,
    extension: Extension,
}

impl<Request> Context<Request> {
    #[inline]
    pub fn get_extension(&self) -> &Extension {
        &self.extension
    }

    #[inline]
    pub fn get_mut_extension(&mut self) -> &mut Extension {
        &mut self.extension
    }

    #[inline]
    pub fn get_request(&self) -> &Request {
        &self.request
    }

    #[inline]
    pub fn get_mut_request(&mut self) -> &mut Request {
        &mut self.request
    }

    pub async fn authenticate<H>(&self, state: H::State) -> Result<H::Response, H::Error>
    where
        H: Handler<Request> + Clone + Sync,
    {
        // TODO: log trace and record metrics as well

        // Get the handler and perform authentication
        // Handle error perfectly
        let handler = self.extension.get::<H>().unwrap();
        handler.authenticate(self, state).await
    }

    pub async fn challenge<H>(&self, state: H::State) -> Result<H::Response, H::Error>
    where
        H: Handler<Request> + Clone + Sync,
    {
        // TODO: log trace and record metrics as well

        // Get the handler and perform authentication
        // Handle error perfectly
        let handler = self.extension.get::<H>().unwrap();
        handler.challenge(self, state).await
    }

    pub async fn forbid<H>(&self, state: H::State) -> Result<H::Response, H::Error>
    where
        H: Handler<Request> + Clone + Sync,
    {
        // TODO: log trace and record metrics as well

        // Get the handler and perform authentication
        // Handle error perfectly
        let handler = self.extension.get::<H>().unwrap();
        handler.forbid(self, state).await
    }

    pub async fn sign_in<H>(&self, state: H::State, payload: H::Payload) -> Result<H::Response, H::Error>
    where
        H: SignInHandler<Request> + Clone + Sync,
    {
        // TODO: log trace and record metrics as well

        // Get the handler and perform authentication
        // Handle error perfectly
        let handler = self.extension.get::<H>().unwrap();
        handler
            .sign_in(self, state, payload)
            .await
    }

    pub async fn sign_out<H>(&self, state: H::State) -> Result<H::Response, H::Error>
    where
        H: SignOutHandler<Request> + Clone + Sync,
    {
        // TODO: log trace and record metrics as well

        // Get the handler and perform authentication
        // Handle error perfectly
        let handler = self.extension.get::<H>().unwrap();
        handler.sign_out(self, state).await
    }
}

#[cfg(feature = "axum")]
const _: () = {
    use axum::{
        extract::FromRequestParts,
        response::{IntoResponse, Response},
    };
    use http::request::Parts;

    #[derive(Debug)]
    pub struct MissingContext;

    impl IntoResponse for MissingContext {
        fn into_response(self) -> Response {
            use http::StatusCode;

            StatusCode::INTERNAL_SERVER_ERROR.into_response()
        }
    }

    impl<S> FromRequestParts<S> for Context<Parts>
    where
        S: Send + Sync,
    {
        type Rejection = MissingContext;

        async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
            parts
                .extensions
                .get::<Self>()
                .cloned()
                .ok_or_else(|| {
                    error!("Codora: Context not initialized. Did you forget to add `ContextBuilder` layer?");
                    MissingContext
                })
        }
    }
};

#[cfg(feature = "axum")]
#[cfg(test)]
mod axum_tests {
    use anyhow::Result;
    use axum::{
        Router,
        body::Body,
        response::{IntoResponse, Response},
        routing::get,
    };
    use http::{Request, StatusCode, request::Parts};
    use tower::ServiceExt;

    use crate::{
        authentication::handler::{CookieHandlerExt as _, CookiePayload, CookieState},
        context::{CFrameworkSecurity, Context},
    };

    #[tokio::test]
    async fn test_context_if_it_compiles_it_works() -> Result<()> {
        let app = Router::new()
            .route("/", get(|context: Context<Parts>| async { () }))
            .layer(CFrameworkSecurity::default());

        let req = Request::builder()
            .uri("/")
            .body(Body::empty())?;

        let res = app.oneshot(req).await?;
        assert_eq!(res.status(), StatusCode::OK);
        Ok(())
    }

    #[tokio::test]
    async fn test_cookie_sign_in_with_axum() -> Result<()> {
        let cf = CFrameworkSecurity::default();

        async fn authentication_handler(ctx: Context<Parts>) -> Result<Response, Response> {
            // Sign in with cookie

            // cookie response
            let res = ctx
                .sign_in_with_cookie(CookieState {}, CookiePayload {})
                .await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR.into_response())?;

            Ok::<_, Response>((res, StatusCode::OK).into_response())
        }

        let app = Router::new()
            .route("/", get(authentication_handler))
            .layer(cf);

        let req = Request::builder()
            .uri("/")
            .body(Body::empty())?;

        let res = app.oneshot(req).await?;
        assert_eq!(res.status(), StatusCode::OK);

        Ok(())
    }
}
