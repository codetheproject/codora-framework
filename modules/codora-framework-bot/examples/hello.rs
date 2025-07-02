use error_stack::Report;
use http_body_util::Full;
use hyper::{Request, Response, body::Bytes, server::conn::http1, service::service_fn};
use hyper_util::rt::TokioIo;
use std::convert::Infallible;
use tokio::net::TcpListener;

#[macro_use]
extern crate tracing;

#[derive(Debug, thiserror::Error)]
#[error("{0}")]
pub enum Error {
    IO(#[from] std::io::Error),
}

pub trait ErrorExt {
    type Output;
    fn to_reportable_error(self) -> Self::Output;
}

impl ErrorExt for std::io::Error {
    type Output = error_stack::Report<Error>;
    fn to_reportable_error(self) -> Self::Output {
        Report::new(Error::IO(self))
    }
}

impl<T> ErrorExt for Result<T, std::io::Error> {
    type Output = Result<T, error_stack::Report<Error>>;
    fn to_reportable_error(self) -> Self::Output {
        self.map_err(|err| Report::new(Error::IO(err)))
    }
}

pub type Result<T, E = error_stack::Report<Error>> = core::result::Result<T, E>;

async fn handler(_: Request<hyper::body::Incoming>) -> Result<Response<Full<Bytes>>, Infallible> {
    Ok(Response::new(Full::new(Bytes::from("Hello, World!"))))
}

pub async fn start_telegram_bot_server() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:3000")
        .await
        .to_reportable_error()?;
    trace!("Started listening on port: 3000");

    loop {
        // handle this error perfectly
        let (stream, addr) = listener
            .accept()
            .await
            .to_reportable_error()?;
        trace!("Accepted connection from: {}", addr);
        let io = TokioIo::new(stream);

        tokio::spawn(async move {
            if let Err(err) = http1::Builder::new()
                .serve_connection(io, service_fn(handler))
                .await
            {
                error!("Error serving connection: {}", err);
            }
        });
    }
}

pub fn main() {}

// implement hello example
