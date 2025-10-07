use crate::{
    ContextBuilder,
    authentication::{Context, ContextHandlerExtension},
    cookie::CookieResponse,
};
use axum::{
    extract::{FromRequestParts, Request},
    response::{IntoResponse, IntoResponseParts, Response, ResponseParts},
};
use http::{StatusCode, request::Parts};
use std::{pin::Pin, sync::Arc, task::Poll};
use tower_service::Service;

#[derive(new, Clone)]
pub struct ContextService<S> {
    inner: S,
    handler_store: Arc<ContextHandlerExtension>,
}

impl<S> Service<Request> for ContextService<S>
where
    S: Service<Request, Response = Response> + Send + 'static,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    // Replace this with ResponseFuture no
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, cx: &mut std::task::Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, request: Request) -> Self::Future {
        // we wanna populate context here
        let (mut parts, body) = request.into_parts();
        // We wanna authenticate here and do other good stuff here
        // create context and insert extension into context
        let res = parts
            .extensions
            // issues: if part.extension is updated would it bubble up to the parts up there i believe it should work like that
            // Should that not work we will find a way to create extension up here that could be shared with the request or context
            .insert(Context::new(parts.clone(), Arc::clone(&self.handler_store)));
        debug_assert!(res.is_none(), "Context is already in extension");

        // we wanna authenticate here
        // for handler in self.hanler_store.get_ref_handler() {

        //     // handler error as you should we gonna get this done later
        //     handler.authenticate()
        // }

        // Rebuild request here after using the parts
        let request = Request::from_parts(parts, body);
        let future = self.inner.call(request);
        Box::pin(async move {
            let response: Response = future.await?;
            Ok(response)
        })
    }
}

impl<S> tower_layer::Layer<S> for ContextBuilder<Arc<ContextHandlerExtension>> {
    type Service = ContextService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        ContextService::new(inner, Arc::clone(&self.handler_store))
    }
}

impl IntoResponseParts for CookieResponse {
    type Error = StatusCode;

    fn into_response_parts(self, res: ResponseParts) -> Result<ResponseParts, Self::Error> {
        todo!()
    }
}

#[derive(Debug)]
pub struct MissingContext;

impl IntoResponse for MissingContext {
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR).into_response()
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

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use axum::{Router, body::Body, response::IntoResponse, routing::get};
    use http::{Request, StatusCode, request::Parts};
    use tower::ServiceExt;

    use crate::{
        Context, ContextBuilder,
        cookie::{CookieHandler, CookieHandlerExt as _, CookieOption, CookiePayload, CookieState},
    };

    #[tokio::test]
    async fn test_context() -> Result<()> {
        let context_builder = ContextBuilder::builder().build();

        let app = Router::new()
            .route("/", get(|_ctx: Context<Parts>| async { "Hello World" }))
            .layer(context_builder);

        let req = Request::builder()
            .uri("/")
            .body(Body::empty())?;

        let res = app.oneshot(req).await?;
        assert_eq!(res.status(), StatusCode::OK);
        // assert!(res.body().);
        Ok(())
    }

    #[allow(irrefutable_let_patterns)]
    #[tokio::test]
    async fn test_cookie_sign_in_with_axum() -> Result<()> {
        let context_builder = ContextBuilder::builder()
            .register_handler(CookieHandler::new(CookieOption::new()))
            .build();

        let app = Router::new()
            .route(
                "/",
                get(|ctx: Context<Parts>| async move {
                    // Sign in with cookie

                    if let Ok(res) = ctx
                        .sign_in_with_cookie(CookieState {}, CookiePayload {})
                        .await
                    {
                        return (res, "We got in").into_response();
                    }

                    "Hello World".into_response()
                }),
            )
            .layer(context_builder);

        let req = Request::builder()
            .uri("/")
            .body(Body::empty())?;

        let res = app.oneshot(req).await?;
        assert_eq!(res.status(), StatusCode::OK);

        Ok(())
    }
}
