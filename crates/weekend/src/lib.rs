// Silence the noise in development!
#![cfg_attr(debug_assertions, allow(dead_code, unused_variables))]
#[cfg(feature = "authentication")]
mod authentication;
#[cfg(feature = "authorization")]
mod authorization;
