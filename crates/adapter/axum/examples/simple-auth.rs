use axum::{debug_handler, routing::get};
use codora_util::string;
use tokio::net::TcpListener;

#[debug_handler]
// () is a Temporary Response
// Context will hold previous claim and current claim which would all be used when authenticating!
async fn handler() -> Result<((), String), &'static str> {
    Ok((res, string!("ok!")))
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let auth = AuthenticationLayer::new();

    let app = axum::Router::new()
        .route("/sign-in", get(handler))
        .layer(auth);

    let listener = TcpListener::bind("127.0.0.1:2345").await?;
    axum::serve(listener, app).await?;
    Ok(())
}
