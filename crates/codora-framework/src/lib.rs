//! Codora
//!
//! What's Codora ?
//!
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

#[cfg(feature = "security")]
pub mod security {
    ///This expose authentication api
    pub mod authentication {
        pub use codora_framework_security::codoraframeworksecurity::{Handler, SignInHandler, SignOutHandler};
    }
}

#[cfg(any(feature = "bot-telegram", feature = "bot-discord"))]
pub mod bot {
    pub use codora_framework_bot::bot::{
        Bot,
        context::{BotContext, BotContextBuilder, Error, Result},
    };

    #[cfg(feature = "bot-telegram")]
    pub mod telegram {
        pub use codora_framework_bot::adapter::telegram::*;
    }

    #[cfg(feature = "bot-discord")]
    pub mod discord {
        pub use codora_framework_bot::adapter::discord::*;
    }
}

#[cfg(feature = "identity")]
pub mod identity {}

pub mod prelude {
    //! Using this prelude this allow user to configure codora without imports clutter
    // #[cfg(feature = "security")]
    // pub use codora_framework_security::{Context, ContextBuilder, ContextHandlerExtension};
}
