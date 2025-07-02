#![forbid(unsafe_code)]
// Silence the noise in development!
#![cfg_attr(debug_assertions, allow(dead_code, unused_variables, warnings))]
// Docs and linting rules
#![cfg_attr(docsrs, feature(doc_auto_cfg, doc_cfg))]
#![cfg_attr(test, allow(clippy::float_cmp))]
#![cfg_attr(not(test), deny(clippy::print_stdout, clippy::dbg_macro))]
// - Lint for missing docs
#![cfg_attr(not(debug_assertions), deny(missing_docs))]

#[cfg(feature = "axum")]
pub mod axum {
    pub use codora_framework_axum::*;
}

#[cfg(not(any(feature = "axum", feature = "others")))]
compile_error!("You must enable exactly one of the features: `axum`, `others`");
