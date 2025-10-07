//! Codora - Security
//!
//! # Overview
//!
//! # TODO
#![forbid(unsafe_code)]
// Silence the noise in development!
#![cfg_attr(debug_assertions, allow(dead_code, unused_variables, unused_imports))]
// Docs and linting rules
#![cfg_attr(docsrs, feature(doc_auto_cfg, doc_cfg))]
#![cfg_attr(test, allow(clippy::float_cmp))]
#![cfg_attr(not(test), deny(clippy::print_stdout, clippy::dbg_macro))]
// - Lint for missing docs
#![cfg_attr(not(debug_assertions), deny(missing_docs))]

// Extern crate
#[macro_use]
extern crate tracing;
#[macro_use]
extern crate derive_new;

pub mod domain {
    mod value {
        mod email;
        mod pii;
    }

    pub use value::*;
}
pub mod email;
pub mod user;

// use anyhow::anyhow;
// use axum::{
//     Router,
//     body::Body,
//     http::Request,
//     response::{IntoResponse, Response},
//     routing::get,
// };
// use codora_framework::{Context, Startup, StartupError};
// use std::{convert::Infallible, pin::Pin, sync::Arc};
// use tokio::{net::TcpListener, runtime::Runtime};
// use tower::Service;

// struct WebServiceFuture {}

// impl Future for WebServiceFuture {
//     type Output = Result<Response, Infallible>;

//     fn poll(self: Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
//         todo!()
//     }
// }

// // implement tower_service::Service for WebService this wouled allow us to inject Webservice into AxumService built by Router
// impl tower_service::Service<Request<Body>> for WebService {
//     type Response = Response;
//     type Error = Infallible;

//     // Fix this later but that's the idea;
//     type Future = WebServiceFuture;

//     fn poll_ready(&mut self, cx: &mut std::task::Context<'_>) -> std::task::Poll<Result<(), Self::Error>> {
//         std::task::Poll::Ready(Ok(()))
//     }

//     fn call(&mut self, req: Request<Body>) -> Self::Future {
//         WebServiceFuture {}
//     }
// }
