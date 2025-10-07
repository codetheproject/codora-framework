use crate::codoraframeworksecurity::{CFrameworkSecurity, Extension};
use axum::{
    extract::{FromRequestParts, Request},
    response::Response,
};
use http::{StatusCode, request::Parts};
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

    fn call(&mut self, mut request: Request) -> Self::Future {
        // Insert a CframeworkSecurity back inside the extension
        let prev = request
            .extensions_mut()
            .insert(CFrameworkSecurity::new(self.extension.clone()));
        debug_assert!(prev.is_none(), "Context already present in request extensions");
        ServiceResponseFuture::new(self.inner.call(request))
    }
}

impl<S> tower_layer::Layer<S> for CFrameworkSecurity {
    type Service = CFrameworkService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        CFrameworkService::new(inner, self.extension().clone())
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

impl<S> FromRequestParts<S> for CFrameworkSecurity
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, _: &S) -> Result<Self, Self::Rejection> {
        parts
            .extensions
            .get::<CFrameworkSecurity>()
            .cloned()
            .ok_or_else(|| {
                //TODO: Update this error message later
                error!("Can't extract CFrameworkSecurity. Is `CFrameworkSecurity` added as layer in your `Router`");
                (StatusCode::INTERNAL_SERVER_ERROR, "Unknown error occured, Please check the log!")
            })
    }
}

#[cfg(test)]
mod axum_tests {
    use crate::prelude::*;
    use anyhow::Result;
    use axum::{
        Router,
        body::Body,
        extract::Request,
        response::{IntoResponse, Response},
        routing::{get, post},
    };
    use http::{StatusCode, request::Parts};
    use tower::ServiceExt;

    #[allow(dead_code)]
    #[derive(Debug, serde::Deserialize)]
    pub struct JwtBody {
        token: String,
    }

    #[tokio::test]
    async fn test_context_if_it_compiles_it_works() -> Result<()> {
        let app = Router::new()
            .route("/signup", get(|_cf_security: CFrameworkSecurity| async { () }))
            .layer(CFrameworkSecurity::default());

        let req = Request::builder()
            .uri("/")
            .body(Body::empty())?;

        let res = app.oneshot(req).await?;
        assert_eq!(res.status(), StatusCode::OK);
        Ok(())
    }

    #[tokio::test]
    async fn test_cookie_sign_in_with_axum_parts() -> Result<()> {
        let cf = CFrameworkSecurity::default().add_cookie(|option| {
            // setup option here

            option
        });

        async fn authentication_handler(parts: Parts, mut ctx: CFrameworkSecurity) -> Result<Response, Response> {
            // cookie response
            let _response = ctx
                .with(parts)
                .sign_in(CookieState {}, CookiePayload {})
                .await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR.into_response())?;

            Ok(String::from("Yet to be implemented").into_response())
        }

        let app = Router::new()
            .route("/", post(authentication_handler))
            .layer(cf);

        let req = Request::builder()
            .uri("/")
            .body(Body::empty())?;

        let res = app.oneshot(req).await?;
        assert_eq!(res.status(), StatusCode::OK);

        Ok(())
    }

    #[tokio::test]
    async fn test_cookie_sign_in_with_axum_typed_request() -> Result<()> {
        let cf = CFrameworkSecurity::default().add_cookie(|option| {
            // setup option here

            option
        });

        async fn authentication_handler(parts: Parts, mut ctx: CFrameworkSecurity) -> Result<Response, Response> {
            let _response = ctx
                .with(parts)
                .sign_in(CookieState {}, CookiePayload {})
                .await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR.into_response())?;

            Ok(String::from("Yet to be implemented").into_response())
        }

        let app = Router::new()
            .route("/", post(authentication_handler))
            .layer(cf);

        let req = Request::builder()
            .uri("/")
            .body(Body::empty())?;

        let res = app.oneshot(req).await?;
        assert_eq!(res.status(), StatusCode::OK);

        Ok(())
    }
}
