#![forbid(unsafe_code)]
// Silence the noise in development!
#![cfg_attr(debug_assertions, allow(dead_code, unused_variables))]
// Docs and linting rules
#![cfg_attr(docsrs, feature(doc_auto_cfg, doc_cfg))]
#![cfg_attr(test, allow(clippy::float_cmp))]
#![cfg_attr(not(test), deny(clippy::print_stdout, clippy::dbg_macro))]
// - Lint for missing docs
#![cfg_attr(not(debug_assertions), deny(missing_docs))]
#![doc = include_str!("../../../README.md")]

#[cfg(not(any(feature = "core", feature = "security")))]
compile_error!("You must enable at least one feature to use codora");

#[cfg(feature = "core")]
pub use codora_core::*;

// TODO -> Inline docs, setup getcodora, learn about action, update readme, changelog, otherstuff
#[cfg(feature = "security")]
pub mod security {
    //!
    //! This is the security module for codora
}
