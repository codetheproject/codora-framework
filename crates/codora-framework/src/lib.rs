//! Codora
//!
//! What's Codora ?
//!
//! Codora is a framework that sped up rust development with lot of features

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

#[macro_use]
extern crate derive_new;

/// Codora Framework
#[derive(Debug)]
pub struct CodoraFramework<S> {
    service: S,
}

impl<S> CodoraFramework<S> {
    pub fn new(service: S) -> Self {
        Self { service }
    }
}

#[cfg(test)]
mod tests {
    use crate::CodoraFramework;

    #[test]
    fn test_codora_framework() {
        let cf = CodoraFramework::new(());
    }
}
