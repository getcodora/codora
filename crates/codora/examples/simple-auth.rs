use axum::{debug_handler, routing::get};
use codora::string;
use codora_security::authentication::{Authentication, Context, claim::Claim};
use tokio::net::TcpListener;

#[derive(Clone)]
struct Codora;

mod codora_parts {
    use axum::{
        extract::FromRequestParts,
        http::{StatusCode, request::Parts},
    };

    impl<S> FromRequestParts<S> for super::Codora
    where
        S: Sync,
    {
        type Rejection = StatusCode;
        async fn from_request_parts(_parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
            todo!()
        }
    }
}

impl<T> Authentication<T> for Codora {
    type Forbid;
    type SignIn;
    type Signout;
    type Challenge;
    type Authenticate;
}

#[debug_handler]
// () is a Temporary Response
// Context will hold previous claim and current claim which would all be used when authenticating!
async fn handler(ctx: Context<Codora>) -> Result<((), String), &'static str> {
    let res = ctx
        .sign_in(&Claim::default())
        .await
        .map_err(|_| "We got error!")?;

    Ok((res, string!("ok!")))
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app = axum::Router::new().route("/sign-in", get(handler));

    let listener = TcpListener::bind("127.0.0.1:2345").await?;
    axum::serve(listener, app).await?;
    Ok(())
}
