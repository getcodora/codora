pub(crate) mod bearer;
pub(crate) mod cookie;
pub(crate) mod jwt;
pub(crate) mod oauth;

use std::future::Future;

use super::{Context, claim::Claim};

pub trait Handler<Request> {
    type Error;

    /// The name of the handler
    /// This is used to identify the handler in logs and other contexts
    const NAME: &'static str;

    /// Authenticate the current request
    ///
    /// This method is called to authenticate the current request
    fn authenticate(&self, contex: &Context<Request>, claim: &Claim) -> impl Future<Output = Result<(), Self::Error>> + Send;

    /// Forbid the current request
    ///
    /// This method is called to forbid the current request
    ///
    /// # Arguments
    /// `state` - The current state of the request `S`
    fn forbid(&self, contex: &Context<Request>, claim: &Claim) -> impl Future<Output = Result<(), Self::Error>> + Send;

    /// Challenge the current request
    ///
    /// This method is called to challenge the current request
    ///
    /// # Arguments
    /// `state` - The current state of the request `Self::State`
    fn challenge(&self, contex: &Context<Request>, claim: &Claim) -> impl Future<Output = Result<(), Self::Error>> + Send;
}

pub mod service {
    use crate::authentication::{Context, claim::Claim};

    pub struct Handler<H> {
        handler: H,
    }

    impl<H> Handler<H> {
        async fn sign_out<Request>(&self, req: &Context<Request>, claim: &Claim) -> Result<(), H::Error>
        where
            H: super::Handler<Request>,
        {
            // Delegate the Request to the handler
            todo!()
        }
    }
}
