#[cfg(feature = "bearer")]
pub mod bearer;
#[cfg(feature = "cookie")]
pub mod cookie;
#[cfg(feature = "jwt")]
pub mod jwt;
#[cfg(feature = "oauth")]
pub mod oauth;

//
pub mod u_auth;

use async_trait::async_trait;
use derive_new::new;
use http::{Request, Uri};
use std::collections::HashMap;
use std::task::{Context, Poll};
use tower_layer::Layer;
use tower_service::Service;

#[async_trait]
pub trait Authentication<Request> {
    type Claim;
    type Error;

    // This is useful for error handling
    const NAME: &'static str;

    /// Authenticate the current Request
    async fn authenticate(&self, req: Request) -> Result<(), ()>;

    //  We might need to return something that can be turn to response cause we are not mutating response
    /// Forbid the current request
    async fn forbid(&self, req: Request, state: State);

    /// Challenge the current request
    async fn challenge(&self, req: Request, state: State);

    /// Sign in the current Request
    async fn sign_in(&self, req: Request, claims: Self::Claim, state: State) -> Result<(), ()>;

    /// Sign out the current Request
    async fn sign_out(&self, req: Request, claims: Self::Claim, state: State) -> Result<(), ()>;
}

#[derive(Clone, Debug, Default)]
pub struct State {
    pub allow_refresh: bool,
    pub is_persistent: bool,
    pub redirect_url: Uri,

    // Store unnamed state
    _state: HashMap<String, String>,
    //
}

impl State {
    pub fn set_url(&mut self, url: Uri) -> &mut Self {
        self.redirect_url = url;
        self
    }

    //  Return Self allow chaining and setting multiple values at once
    pub fn set_state(&mut self, key: &str, value: &str) -> &mut Self {
        self._state.insert(key.into(), value.into());
        self
    }

    pub fn get_state(&self, key: &str) -> Option<&String> {
        self._state.get(key)
    }

    pub fn set_all<'a, T>(&mut self, values: T) -> &mut Self
    where
        T: IntoIterator<Item = (&'a str, &'a str)>,
    {
        for (key, value) in values.into_iter() {
            self.set_state(key, value);
        }

        self
    }
}

impl From<&Uri> for State {
    fn from(value: &Uri) -> Self {
        let redirect_url = value.to_owned();

        State {
            redirect_url,
            ..State::default()
        }
    }
}

#[derive(new)]
pub struct AuthenticationService<S, T> {
    inner: S,
    // Other default service will be added later
    state: State,
    auth_extensions: Vec<T>,
}

pub struct AuthenticationLayer<T> {
    // Other default service will be added later
    state: State,
    auth_extensions: Vec<T>,
}

impl<S, T, ReqBody> Service<Request<ReqBody>> for AuthenticationService<S, T>
where
    S: Service<Request<ReqBody>>,
    T: Authentication<Request<ReqBody>> + Clone + Send + Sync + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut req: Request<ReqBody>) -> Self::Future {
        let ext = req.extensions_mut();
        ext.insert(self.state.clone());

        while let Some(service) = self.auth_extensions.iter().next() {
            // T::NAME;

            ext.insert(service.clone());
        }

        self.inner.call(req)
    }
}

impl<S, T> Layer<S> for AuthenticationLayer<T>
where
    T: Clone,
{
    type Service = AuthenticationService<S, T>;

    fn layer(&self, inner: S) -> Self::Service {
        AuthenticationService::new(inner, self.state.clone(), self.auth_extensions.clone())
    }
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn state() {
        let mut state = State::default();

        state.set_all([("Login", "true")]);

        if let Some(value) = state.get_state("Login") {
            assert_eq!(value, "true");
        }
    }

    #[test]
    fn from_uri() -> anyhow::Result<()> {
        let url = "http://localhost:3000".parse::<Uri>()?;

        let mut state = State::from(&url);
        state.set_state("is_created", "true");
        println!("{:#?}", state);

        if let Some(value) = state.get_state("is_crated") {
            assert_eq!(value, "true");
        }

        Ok(())
    }
}
