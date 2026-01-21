use axum::{Extension, Router, body::Body, response::IntoResponse, routing::get};
use http::{Request, request::Parts};
use tower::ServiceExt;
use valar_core::valar::{Valar, context::AuthenticationContext, handler::State};
use valar_handler::cookie::handler::CookieHandler;

#[tokio::test]
async fn test_with_axum() -> anyhow::Result<()> {
    let valar = Valar::default();
    let app = Router::new()
        .route(
            "/",
            get(|Extension(mut valar): Extension<Valar>, part: Parts| async move {
                let _response = valar
                    .auth(part)
                    .authenticate::<CookieHandler>(&30 as &dyn State)
                    .await;

                ().into_response()
            }),
        )
        .layer(Extension(valar));

    let request = Request::builder()
        .uri("/")
        .method("GET")
        .body(Body::empty())?;

    let _response = app.oneshot(request).await;

    Ok(())
}
