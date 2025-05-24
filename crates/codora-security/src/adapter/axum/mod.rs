//! This should explain how codora-security used axum
//!
//!
//!
//!
//!
//!
//!
//!  More Docs!
use std::sync::{Arc, RwLock};

use crate::authentication::{Authentication, Context, claim::Claim};
use axum::{
    extract::{FromRequestParts, Request},
    http::{StatusCode, request::Parts},
};

impl<P, S> FromRequestParts<S> for Context<P>
where
    S: Sync,
    P: Authentication<Request> + FromRequestParts<S>,
    <P as FromRequestParts<S>>::Rejection: std::fmt::Debug,
{
    type Rejection = StatusCode;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let provider = P::from_request_parts(parts, state)
            .await
            .unwrap();

        let claim = Arc::new(RwLock::new(Claim::default()));
        Ok(Context::new(claim, provider))
    }
}

// Impl IntoResponse and IntoResponseParts
