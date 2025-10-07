use axum::{Extension, Router, body::Body, routing::get};
use http::{Request, request::Parts};
use tower::ServiceExt;
use valar_core::valar::Valar;
use valar_handler::cookie::{handler::CookieHandler, state::CookieState};

#[tokio::test]
async fn test_with_axum() -> anyhow::Result<()> {
    let valar = Valar::default();
    let app = Router::new()
        .route(
            "/",
            get(|Extension(mut valar): Extension<Valar>, part: Parts| async move {
                let _response = valar
                    .ctx(&part)
                    .authenticate::<CookieHandler>(CookieState {})
                    .await;

                ()
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
