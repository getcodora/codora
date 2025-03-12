use crate::security::Provider;

pub struct Context<S, P>
where
    P: Provider<S>,
{
    state: S,
    provider: P,
    // This will basically hold a context we don't know if we should support multiple context tho
    // context: Arc<Box<dyn >>
}

async fn use_context<S, P>(ctx: Context<S, P>, state: S)
where
    P: Provider<S>,
{
    let signin = ctx
        .provider
        .get_sign_in(String::new())
        .await;

    // let _ = signin
    //     .sign_in(&state, &String::new())
    //     .await;
}

pub mod handler {
    use async_trait::async_trait;

    #[async_trait]
    pub trait Handler<State> {
        type Error;

        /// The name of the handler
        /// This is used to identify the handler in logs and other contexts
        const NAME: &'static str;

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
    }
}

pub mod sign_out {
    use super::handler::Handler;
    use async_trait::async_trait;

    #[async_trait]
    pub trait SignOutHandler<State>: Handler<State> {
        type Claim;

        async fn sign_out(&self, state: &State, claim: &Self::Claim) -> Result<(), Self::Error>;
    }

    pub struct SignOutContext<State, Handler: SignOutHandler<State>> {
        handler: Handler,
        before_signout: Box<dyn Fn(&State, &Handler::Claim)>,
        after_signout: Box<dyn Fn(&State, &Handler::Claim)>,
    }
}

pub mod sign_in {
    use super::sign_out::SignOutHandler;
    use async_trait::async_trait;

    #[async_trait]
    pub trait SignInHandler<State>: SignOutHandler<State> {
        async fn sign_in(&self, state: &State, claim: &Self::Claim) -> Result<(), Self::Error>;
    }

    pub struct SignInContext<State, Handler: SignOutHandler<State>> {
        handler: Handler,
        before_signout: Box<dyn Fn(&State, &Handler::Claim)>,
        after_signout: Box<dyn Fn(&State, &Handler::Claim)>,
    }
}

#[cfg(test)]
mod test {}
