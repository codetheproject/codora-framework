//! Valar
//!
//! What's Valar ?
//!
#![forbid(unsafe_code)]
// Silence the noise in development!
// #![cfg_attr(debug_assertions, allow(dead_code, unused_variables, unused_imports))]
// Docs and linting rules
#![cfg_attr(docsrs, feature(doc_auto_cfg, doc_cfg))]
#![cfg_attr(test, allow(clippy::float_cmp))]
#![cfg_attr(not(test), deny(clippy::print_stdout, clippy::dbg_macro))]
// - Lint for missing docs
#![cfg_attr(not(debug_assertions), deny(missing_docs))]
#![doc = include_str!("../README.md")]

pub mod http;
pub mod identity;
pub mod metrics;
pub mod valar;

#[macro_use]
extern crate derive_new;
#[macro_use]
extern crate tracing;

pub use valar::handler::resolve_state;
pub type BoxFuture<'a, T> = std::pin::Pin<Box<dyn Future<Output = T> + Send + Sync + 'a>>;
