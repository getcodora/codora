use std::any::TypeId;

use anyhow::Result;
use axum::{
  extract::{Path, Request},
  middleware::{from_fn, Next},
  response::Response,
  routing::get,
  Extension, Router,
};
use axum_extra::routing::Resource;
use plus::std_rs::{f, new, string};
use tokio::net::TcpListener;

#[derive(Clone, Debug, new)]
struct State {
  user: String,
}

async fn debug_extenstion(request: Request, next: Next) -> Response {
  println!("Extension: {:?}", TypeId::of::<Extension<State>>());
  next.run(request).await
}

#[tokio::main]
async fn main() -> Result<()> {
  let listerner = TcpListener::bind("0.0.0.0:3000").await?;

  let resources = Resource::named("users")
    .index(|state: Extension<State>| async move { format!("Hello, {}!", state.user) })
    .new(|path: Path<String>| async move { f!("Hello, {}", path.0) });

  let app = Router::new()
    .merge(resources)
    .route("/", get(|state: Extension<State>| async move { format!("Hello, {}!", state.user) }))
    .layer(Extension(State::new(string!("West"))))
    .layer(from_fn(debug_extenstion));

  axum::serve(listerner, app).await?;
  Ok(())
}
