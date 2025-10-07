#![forbid(unsafe_code)]
// Silence the noise in development!
#![cfg_attr(debug_assertions, allow(dead_code, unused_variables, warnings))]
// Docs and linting rules
#![cfg_attr(docsrs, feature(doc_auto_cfg, doc_cfg))]
#![cfg_attr(test, allow(clippy::float_cmp))]
#![cfg_attr(not(test), deny(clippy::print_stdout, clippy::dbg_macro))]
// - Lint for missing docs
#![cfg_attr(not(debug_assertions), deny(missing_docs))]

#[macro_use]
extern crate derive_new;
#[macro_use]
extern crate tracing;

pub mod adapter;
mod codoraframeworkbot;

#[cfg(feature = "codora-framework-bot-telegram")]
use codora_framework_macro::Command;
