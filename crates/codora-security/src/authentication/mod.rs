use claim::Claim;
use handler::Handler;
use sign_in::SignInHandler;
use sign_out::SignOutHandler;
use std::sync::{Arc, RwLock};

pub mod claim;
pub mod handler;

#[derive(Clone)]
pub struct Context<P> {
    claim: Arc<RwLock<Claim>>,
    provider: P,
}

impl<P> Context<P> {
    pub fn new(claim: Arc<RwLock<Claim>>, provider: P) -> Self {
        Self { claim, provider }
    }
}

impl<P> Context<P>
where
    P: Authentication<String>,
{
    pub async fn sign_in(&self, claim: &Claim) -> Result<(), ()> {
        // let sign_in = self.provider.;

        todo!()
    }

    pub async fn sign_out(&self, claim: &Claim) -> Result<(), ()> {
        todo!()
    }
    pub async fn forbid(&self, claim: &Claim) -> Result<(), ()> {
        todo!()
    }
    pub async fn challenge(&self, claim: &Claim) -> Result<(), ()> {
        todo!()
    }
    pub async fn authenticate(&self, claim: &Claim) -> Result<(), ()> {
        todo!()
    }
}

pub trait FromRequest<Request> {
    type Error;
    type Output;

    fn from_request(&self, req: &Request) -> impl Future<Output = Result<Self::Output, Self::Error>>;
}

/// A trait that defines the contract for handling authentication workflows
/// over an incoming `Request` type.
///
/// This trait provides associated types for each of the main steps involved in
/// an authentication system: forbidding, signing in, signing out, issuing challenges,
/// and authenticating. Each associated type is expected to implement `FromRequest`,
/// which means it can extract itself from an incoming `Request`.
///
/// Once extracted, the type produced by `FromRequest::Output` must implement
/// a specific handler trait to perform its task:
///
/// - `Forbid` → produces a `Handler`
/// - `SignIn` → produces a `SignInHandler`
/// - `Signout` → produces a `SignOutHandler`
/// - `Challenge` → produces a `Handler`
/// - `Authenticate` → produces a `Handler`
///
/// ## Associated Types
/// - `type Forbid`: Handles forbidden requests.
/// - `type SignIn`: Handles sign-in requests.
/// - `type Signout`: Handles sign-out requests.
/// - `type Challenge`: Handles authentication challenges (e.g., multi-factor, CAPTCHA).
/// - `type Authenticate`: Handles actual authentication (e.g., verifying tokens).
///
/// ## Trait Bounds
/// - Each associated type implements `FromRequest<Request>`.
/// - The extracted `Output` type implements the appropriate handler trait.
pub trait Authentication<Request>
where
    Self::Forbid: FromRequest<Request>,
    <Self::Forbid as FromRequest<Request>>::Output: Handler,
    Self::SignIn: FromRequest<Request>,
    <Self::SignIn as FromRequest<Request>>::Output: SignInHandler,
    Self::Signout: FromRequest<Request>,
    <Self::Signout as FromRequest<Request>>::Output: SignOutHandler,
    Self::Challenge: FromRequest<Request>,
    <Self::Challenge as FromRequest<Request>>::Output: Handler,
    Self::Authenticate: FromRequest<Request>,
    <Self::Authenticate as FromRequest<Request>>::Output: Handler,
{
    type Forbid;
    type SignIn;
    type Signout;
    type Challenge;
    type Authenticate;
}

pub mod sign_out {
    use super::{claim::Claim, handler::Handler};
    use std::future::Future;

    pub trait SignOutEvent {
        fn after_signout(&self, claim: &Claim);
        fn before_signout(&self, claim: &Claim);
    }

    pub trait SignOutHandler: Handler {
        fn sign_out(&self, claim: &Claim) -> impl Future<Output = Result<(), Self::Error>> + Send;
    }
}

pub mod sign_in {
    use super::{FromRequest, claim::Claim, sign_out::SignOutHandler};
    use std::future::Future;

    pub trait SignInEvent {
        fn after_signin(&self, claim: &Claim);
        fn before_signin(&self, claim: &Claim);
    }
    pub trait SignInHandler: SignOutHandler {
        fn sign_in(&self, claim: &Claim) -> impl Future<Output = Result<(), Self::Error>> + Send;
    }

    pub struct SignInError;
    pub struct SignIn<T>(T);

    impl<T, Request> FromRequest<Request> for SignIn<T>
    where
        T: SignInHandler,
    {
        type Error = SignInError;

        type Output = T;

        async fn from_request(&self, req: &Request) -> Result<Self::Output, Self::Error> {
            todo!()
        }
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn test_context() {}
}
