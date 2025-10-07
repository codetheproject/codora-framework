use axum::Extension;
use tokio::net::TcpListener;
use valar_core::valar::Valar;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let valar = Valar::default();
    let app = axum::Router::new()
        // .route(
        //     "/",
        //     get(|Extension(mut valar): Extension<Valar>, part: Parts| async move {
        //         let _response = valar
        //             .ctx(&part)
        //             .authenticate::<CookieHandler>(CookieState {})
        //             .await;
        //         let _response = valar
        //             .ctx(&part)
        //             .sign_in::<CookieHandler>(CookieState {}, CookiePayload {})
        //             .await;
        //         ()
        //     }),
        // )
        .layer(Extension(valar));

    let listener = TcpListener::bind("127.0.0.1:3000").await?;
    axum::serve(listener, app).await?;
    Ok(())
}
