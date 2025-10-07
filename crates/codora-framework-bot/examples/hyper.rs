use http_body_util::Full;
use hyper::{Request, Response, body::Bytes, server::conn::http2, service::service_fn};
use hyper_util::rt::{TokioExecutor, TokioIo};
use std::{convert::Infallible, pin::pin};
use tokio::net::TcpListener;

async fn handler(_: Request<impl hyper::body::Body>) -> Result<Response<Full<Bytes>>, Infallible> {
    Ok(Response::new(Full::new(Bytes::from("Hello World!"))))
}

#[allow(unreachable_code)]
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();

    let listener = TcpListener::bind("127.0.0.1:3000").await?;

    loop {
        let (io, _) = listener.accept().await?;
        let tokio_io = TokioIo::new(io);
        // let resolve_now = async {};

        tokio::task::spawn(async move {
            let conn = http2::Builder::new(TokioExecutor::new())
                .enable_connect_protocol()
                .serve_connection(tokio_io, service_fn(handler));

            let mut conn = pin!(conn);
            // let mut resolve_now = pin!(resolve_now);

            loop {
                tokio::select! {
                    result = conn.as_mut() =>  {
                        if let Err(_err) = result {
                            tracing::trace!("failed to serve connection: {_err:#}");
                        }
                        break;
                    }
                    // _ = &mut resolve_now => {
                    //     tracing::trace!("signal received in task, starting graceful shutdown");
                    //     conn.as_mut().graceful_shutdown();
                    // }
                }
            }

            Ok::<_, anyhow::Error>(())
        });
    }
    Ok(())
}
