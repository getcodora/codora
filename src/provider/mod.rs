use crate::context::{handler::Handler, sign_in::SignInHandler, sign_out::SignOutHandler};

#[async_trait::async_trait]
pub trait Provider<State> {
    type Forbid: Handler<State>;
    type SignIn: SignInHandler<State>;
    type Signout: SignOutHandler<State>;
    type Challenge: Handler<State>;
    type Authenticate: Handler<State>;

    async fn get_forbid<Request>(&self, req: Request) -> Self::Forbid;
    async fn get_sign_in<Request>(&self, req: Request) -> Self::SignIn;
    async fn get_sign_out<Request>(&self, req: Request) -> Self::Signout;
    async fn get_challenge<Request>(&self, req: Request) -> Self::Challenge;
    async fn get_authenticate<Request>(&self, req: Request) -> Self::Authenticate;
}
