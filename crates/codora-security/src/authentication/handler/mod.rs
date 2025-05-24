mod bearer;
mod cookie;
mod jwt;
mod oauth;

use std::future::Future;

pub trait Handler {
    type Error;

    /// The name of the handler
    /// This is used to identify the handler in logs and other contexts
    const NAME: &'static str;

    /// Authenticate the current request
    ///
    /// This method is called to authenticate the current request
    fn authenticate(&self) -> impl Future<Output = Result<(), Self::Error>> + Send;

    /// Forbid the current request
    ///
    /// This method is called to forbid the current request
    ///
    /// # Arguments
    /// `state` - The current state of the request `S`
    fn forbid(&self) -> impl Future<Output = Result<(), Self::Error>> + Send;

    /// Challenge the current request
    ///
    /// This method is called to challenge the current request
    ///
    /// # Arguments
    /// `state` - The current state of the request `Self::State`
    fn challenge(&self) -> impl Future<Output = Result<(), Self::Error>> + Send;
}
