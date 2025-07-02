#![forbid(unsafe_code)]
// Silence the noise in development!
#![cfg_attr(debug_assertions, allow(dead_code, unused_variables))]
// Docs and linting rules
#![cfg_attr(docsrs, feature(doc_auto_cfg, doc_cfg))]
#![cfg_attr(test, allow(clippy::float_cmp))]
#![cfg_attr(not(test), deny(clippy::print_stdout, clippy::dbg_macro))]
// - Lint for missing docs
// #![cfg_attr(not(debug_assertions), deny(missing_docs))]
// #![doc = include_str!("../../../README.md")]

// TODO: This crate serves as the main entry point for the entire Codora codebase.
// It exposes all public APIs and the main binary will include the Codora CLI for managing codora cloud
