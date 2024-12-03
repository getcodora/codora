use std::future::Future;

use crate::{Authentication, State};

/// An authentication extension that authenticate a request with a bearer token

pub struct BearerOption {
    // Exp_time
    // Alg
}

pub struct Bearer {
    bearer_option: BearerOption, // Implement Authentication
}

#[async_trait::async_trait]
impl Authentication for Bearer {
    type Claim;
    type Error;

    // This is useful for error handling
    const NAME: &'static str = "Bearer";

    /// Authenticate the current Request
    async fn authenticate(&self, req: Request) -> Result<(), Self::Error> {
        todo!()
    }

    //  We might need to return something that can be turn to response cause we are not mutating response
    /// Forbid the current request
    async fn forbid(&self, req: Request, state: State) {
        todo!()
    }

    /// Challenge the current request
    async fn challenge(&self, req: Request, state: State) {
        todo!()
    }

    /// Sign in the current Request
    async fn sign_in(&self, req: Request, claims: Self::Claim, state: State) -> Result<(), ()>;

    /// Sign out the current Request
    async fn sign_out(&self, req: Request, claims: Self::Claim, state: State) -> Result<(), ()>;
}
