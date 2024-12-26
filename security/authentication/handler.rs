use crate::ClaimBucket;

use super::state;
use async_trait::async_trait;
use std::{borrow::Cow, collections::HashMap, ops::Deref};

#[async_trait]
pub trait AuthenticationHandler<C, S>
where
    C: ClaimBucket,
    S: std::ops::Deref<Target = state::State>,
{
    type Error;

    const NAME: &'static str;

    async fn authenticate(&self) -> Result<(), Self::Error>;

    async fn forbid(&self, state: &S) -> Result<(), Self::Error>;

    async fn challenge(&self, state: &S) -> Result<(), Self::Error>;

    async fn sign_in(&self, state: &S, claim: &C) -> Result<(), Self::Error>;

    // Claims is provided because we might want to signout dynamically based on some available state ...
    async fn sign_out(&self, state: &S, claim: &C) -> Result<(), Self::Error>;
}

pub mod mcb {
    //! Memory claim bucket
    //!

    /// Memory Claim Bucket
    pub struct MCB {}
}
