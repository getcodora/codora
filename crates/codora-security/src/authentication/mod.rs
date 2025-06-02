use claim::Claim;
use codora_util::new;
use std::sync::{Arc, RwLock};

pub mod claim;
pub mod handler;

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

pub struct State {}

pub struct AuthenticationBuilder {
    handler: Option<Vec<()>>,
}

// Hot context created on every Request so each Request has it's own context
#[derive(Clone, new)]
pub struct Context<Request> {
    state: Arc<State>,
    claim: Arc<RwLock<Claim>>,
    request: Request,
}

#[cfg(test)]
mod test {

    #[tokio::test]
    async fn test_context() {}
}
