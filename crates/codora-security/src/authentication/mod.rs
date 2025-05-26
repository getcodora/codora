use claim::Claim;
use codora_util::new;
use std::sync::{Arc, RwLock};

pub mod claim;
pub mod handler;

/*
pub struct AuthenticationLayer
*/

#[derive(Clone, Default)]
pub struct Authentication {
    // This is used to monitor all the handler in case of authenticating! and populating the claim on each request
    handler: Option<Arc<Vec<()>>>,
    // This will store non handler state of any T
    state: String,
    // Other stuff here
}

impl Authentication {
    // Get handler
    pub async fn get_handler<'a, T: 'a>(&'a self) -> Option<&'a T> {
        // self.handler.as_ref().map(|handler| handler.get)
        todo!()
    }

    // Get State as well
    pub async fn get_state<'a, T: 'a>(&'a self) -> Option<&'a T> {
        // self.handler.as_ref().map(|handler| handler.get)
        todo!()
    }
}

pub struct AuthenticationBuilder {
    handler: Option<Vec<()>>,
}

// Hot context created on every Request so each Request has it's own context
#[derive(Clone, new)]
pub struct Context<Request> {
    // Replace this with state! we want something that could store any T like extension used in http::Extension!
    /*
    All the state added in this layer are transient which means it's shared among request
    tapped into request run authenticate which populate the context claim!
    let auth = AuthenticationBuilder::new(/Some State/)
        .add_cookie()
        .add_jwt()
        .add_state(|state| state.add(String::new()))
     */
    // THis state should be shared among Context you see
    state: Arc<Authentication>,
    claim: Arc<RwLock<Claim>>,
    req: Request,
}

//
pub mod sign_out {
    use super::{Context, claim::Claim, handler::Handler};
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

pub mod sign_in {
    use super::{Context, claim::Claim, sign_out::SignOutHandler};
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

#[cfg(test)]
mod test {
    use super::{
        Authentication,
        handler::cookie::{CookieAuthenticationExt as _, CookieOption},
    };

    #[tokio::test]
    async fn test_context() {
        let mut auth = Authentication::default().add_cookie(|auth| CookieOption {});
    }
}
