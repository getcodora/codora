#![forbid(unsafe_code)]
// Silence the noise in development!
#![cfg_attr(debug_assertions, allow(dead_code, unused_variables))]
// Docs and linting rules
#![cfg_attr(docsrs, feature(doc_auto_cfg, doc_cfg))]
#![cfg_attr(test, allow(clippy::float_cmp))]
#![cfg_attr(not(test), deny(clippy::print_stdout, clippy::dbg_macro))]
// - Lint for missing docs
#![cfg_attr(not(debug_assertions), deny(missing_docs))]
//!
//! # codora
//!
//! Write some docs about the codora here!
// #![doc = include_str!("../docs/codora.md")]

#[cfg(feature = "util")]
pub use codora_util::*;

// TODO -> Inline docs
#[cfg(feature = "security")]
pub mod security {
    //!
    //! This is the security module for codora
    pub use codora_security::{
        context::{
            handler::Handler,
            sign_in::{SignInContext, SignInHandler},
            sign_out::{SignOutContext, SignOutHandler},
        },
        provider::Provider,
        state::State,
    };
}
