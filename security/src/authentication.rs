use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::{
    borrow::Cow,
    collections::HashMap,
    fmt::Debug,
    ops::Deref,
    sync::{Arc, Mutex},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct State {
    redirect_url: String,
}

#[async_trait]
pub trait Handler<Payload, State> {
    type Error;

    /// The name of the authentication handler
    /// This is used to identify the handler in logs and other contexts
    const NAME: &'static str;

    /// This default to `Self::NAME` used for persisting state in global context
    fn get_name(&self) -> &'static str {
        Self::NAME
    }

    /// Authenticate the current request
    ///
    /// This method is called to authenticate the current request
    async fn authenticate(&self) -> Result<(), Self::Error>;

    /// Forbid the current request
    ///
    /// This method is called to forbid the current request
    ///
    /// # Arguments
    /// `state` - The current state of the request `S`
    async fn forbid(&self, state: &State) -> Result<(), Self::Error>;

    /// Challenge the current request
    ///
    /// This method is called to challenge the current request
    ///
    /// # Arguments
    /// `state` - The current state of the request `Self::State`
    async fn challenge(&self, state: &State) -> Result<(), Self::Error>;

    /// Sign in the current request
    ///
    /// This method is called to sign-in
    ///
    /// # Arguments
    /// `state` - The current state of the request `Self::State`
    /// `payload` - The payload to be used for signing in `Self::Payload`
    async fn sign_in(&self, state: &State, payload: &Payload) -> Result<(), Self::Error>;

    /// Sign out in the current request
    ///
    /// This method is called to sign-in
    ///
    /// # Arguments
    /// `state` - The current state of the request `Self::State`
    /// `payload` - The payload to be used for signing in `Self::Payload`
    async fn sign_out(&self, state: &State, payload: &Payload) -> Result<(), Self::Error>;
}

pub trait Context {}

pub trait HandlerProvider {}

/// Authentication Service
///
/// This is the main entry point for the authentication service that will be used to authenticate the current request   
///
/// # Example
///
#[derive(Clone)]
pub struct Authentication<H> {
    context: Arc<Mutex<Vec<Box<dyn Context>>>>,
    handler: H,
}

pub struct AuthenticationBuilder {}

impl<H> Authentication<H> {
    fn build() -> AuthenticationBuilder {
        todo!()
    }
}

#[cfg(test)]
mod test {}
