#![forbid(unsafe_code)]
// Silence the noise in development!
#![cfg_attr(debug_assertions, allow(dead_code, unused_variables, unused_imports))]
// Docs and linting rules
#![cfg_attr(docsrs, feature(doc_auto_cfg, doc_cfg))]
#![cfg_attr(test, allow(clippy::float_cmp))]
#![cfg_attr(not(test), deny(clippy::print_stdout, clippy::dbg_macro))]
// - Lint for missing docs
#![cfg_attr(not(debug_assertions), deny(missing_docs))]
#![doc = include_str!("../README.md")]

//! Codora
//!
//! What's Codora ?
//!
//! Codora is a framework that sped up rust development with lot of features
//!
//! * Codora  -> abstraction to ensure typed enviroment files or enviroment config files like yaml, json, ini, toml and setup an enviroment to use any of codora service
//! * Codora bot -> Provide abstraction to build telegram or discord bot or any social bot
//! * Codora bucket -> Api to access s3 like storage
//! * Codora identity -> Authentication and Authorization with identity framework
//! * Codora mailer -> Api to send notification via email, sms and other adapter
//! * Codora ws -> Websocket framework built ontop codora and other api
//! * Codora worker -> Background job and queues with third party api
//!
//!
//!
#[cfg(not(debug_assertions))]
compile_error!("This crate or lib is under heavy development please wait for the first release.");

#[macro_use]
extern crate derive_new;
#[cfg(feature = "macros")]
#[macro_use]
extern crate codora_framework_proc_macro;
#[cfg(feature = "conf")]
mod conf;
pub use codora_framework_proc_macro::{Conf, main};
