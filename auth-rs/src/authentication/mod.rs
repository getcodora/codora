use async_trait::async_trait;
use std::{borrow::Cow, collections::HashMap, ops::Deref};

#[async_trait]
pub trait Authentication {
    type Error;

    const NAME: &'static str;

    async fn authenticate(&self) -> Result<(), Self::Error>;

    async fn forbid(&self);

    async fn challenge(&self);

    async fn sign_in(&self) -> Result<(), Self::Error>;

    async fn sign_out(&self) -> Result<(), Self::Error>;
}

pub struct AuthRs {}

#[cfg(test)]
mod test {}
