use crate::{authentication::State, Handler};
use std::ops::Deref;

pub struct Bearer {
    // Option - event
}

pub struct BearerError {}

#[async_trait::async_trait]
impl<P, S> Handler<P, S> for Bearer
where
    S: Deref<Target = State>,
{
    type Error = BearerError;

    /// The name of the authentication handler
    /// This is used to identify the handler in logs and other contexts
    const NAME: &'static str = "bearer";

    /// Authenticate the current request
    ///
    /// This method is called to authenticate the current request
    async fn authenticate(&self) -> Result<(), Self::Error> {
        todo!()
    }

    /// Forbid the current request
    ///
    /// This method is called to forbid the current request
    ///
    /// # Arguments
    /// `state` - The current state of the request `S`
    async fn forbid(&self, state: &S) -> Result<(), Self::Error> {
        todo!()
    }

    /// Challenge the current request
    ///
    /// This method is called to challenge the current request
    ///
    /// # Arguments
    /// `state` - The current state of the request `Self::State`
    async fn challenge(&self, state: &S) -> Result<(), Self::Error> {
        todo!()
    }

    /// Sign in the current request
    ///
    /// This method is called to sign-in
    ///
    /// # Arguments
    /// `state` - The current state of the request `Self::State`
    /// `payload` - The payload to be used for signing in `Self::Payload`
    async fn sign_in(&self, state: &S, payload: &P) -> Result<(), Self::Error> {
        todo!()
    }

    /// Sign out in the current request
    ///
    /// This method is called to sign-in
    ///
    /// # Arguments
    /// `state` - The current state of the request `Self::State`
    /// `payload` - The payload to be used for signing in `Self::Payload`
    async fn sign_out(&self, state: &S, payload: &P) -> Result<(), Self::Error> {
        todo!()
    }
}
