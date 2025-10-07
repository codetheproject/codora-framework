#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // setup authentication and authorization
    // let cf_security_context = cf::security::builder()
    //     .register_handler(CookieHandler::with_default(|default| {
    //         //

    //         default
    //     }))
    //     .freeze();

    // let app = Router::new()
    //     .route("/", get(|| async { return "Hello World" }))
    //     .layer(codora_framework_security_context);

    // let listener = TcpListener::bind("127.0.0.1:4000").await?;
    // axum::serve(listener, app).await?;

    Ok(())
}
