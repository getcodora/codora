#![forbid(unsafe_code)]
// Silence the noise in development!
#![cfg_attr(debug_assertions, allow(dead_code, unused_variables))]
// Docs and linting rules
#![cfg_attr(docsrs, feature(doc_auto_cfg, doc_cfg))]
#![cfg_attr(test, allow(clippy::float_cmp))]
#![cfg_attr(not(test), deny(clippy::print_stdout, clippy::dbg_macro))]
// - Lint for missing docs
#![cfg_attr(not(debug_assertions), deny(missing_docs))]
//! This crate is inspired by the [`validator`](https://crates.io/crates/validator) crate.
//! Some of the code in this crate is copied from the [`validator`](https://crates.io/crates/validator) crate.
//! The `validator` crate is authored by Michael-F-Bryan and maintained by the Rust community.
//!
//! The `validator` crate is licensed under either of:
//! * Apache License, Version 2.0
//! * MIT License
//!
//! For more information, visit the [validator repository](https://github.com/Keats/validator).
#![doc = include_str!("../README.md")]

mod internal;

pub use internal::{IntoError, Validate, ValidationError};

// TODO - re-export the macro so we can use it like this
#[cfg(feature = "derive")]
pub use validator_derive::Validate;
