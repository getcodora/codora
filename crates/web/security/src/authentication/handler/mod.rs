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

pub mod handler {
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

pub mod sign_in {
    use super::sign_out::SignOutHandler;
    use crate::authentication::{Context, claim::Claim};
    use std::future::Future;

    pub trait SignInHandler<Request>: SignOutHandler<Request> {
        type Success;

        fn sign_in(&self, contex: &Context<Request>, claim: &Claim) -> impl Future<Output = Result<Self::Success, Self::Error>> + Send;
    }

    /*
       We want the api to be like

       // Sign in can create it self from request part via state a state populated when the application starts and shared among the request
       // SignIn works with any T as long as T::Option is in State which would be used to create T
       async fn get_users(sign_in_ctx: SignIn<Cookie>, claim: Context<Parts>) -> Response {
           let claim = Claim::default();
           let res = sign_in_ctx.sign_in(context, claim).await?;

           Ok(res)
       }
    */
    #[derive(Clone)]
    pub struct SignIn<H> {
        handler: H,
        // Other Stuff goes in here
    }

    impl<H> SignIn<H> {
        async fn sign_in<Request>(&self, req: &Context<Request>, claim: &Claim) -> Result<H::Success, H::Error>
        where
            H: SignInHandler<Request>,
        {
            // Delegate the Request to the handler
            todo!()
        }
    }
}
//
pub mod sign_out {
    use crate::authentication::{Context, claim::Claim, handler::Handler};
    use std::future::Future;

    pub trait SignOutHandler<Request>: Handler<Request> {
        fn sign_out(&self, ctx: Context<Request>, claim: &Claim) -> impl Future<Output = Result<(), Self::Error>> + Send;
    }

    #[derive(Clone)]
    pub struct SignOut<H> {
        handler: H,
        // Other Stuff goes in here
    }

    impl<H> SignOut<H> {
        async fn sign_out<Request>(&self, req: &Context<Request>, claim: &Claim) -> Result<(), H::Error>
        where
            H: SignOutHandler<Request>,
        {
            // Delegate the Request to the handler
            todo!()
        }
    }
}
