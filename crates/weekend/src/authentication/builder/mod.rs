use std::fmt;
use tower_layer::{Layer, Stack};

/// This is use to setup authentication in a service
///
///  [`AuthenticationBuilder`] provides a
///
/// # Authentication Builder
///
/// TODO Explain it further and refrence tower and tower-service for being a source of inspiration
///
#[derive(Clone)]
pub struct AuthenticationBuilder<L> {
    layer: L,
}

impl<L> AuthenticationBuilder<L> {
    pub fn new(layer: L) -> Self {
        Self { layer }
    }

    fn layer<T>(self, layer: T) -> AuthenticationBuilder<Stack<T, L>> {
        AuthenticationBuilder {
            layer: Stack::new(layer, self.layer),
        }
    }

    /// Returns the underlying [`Layer`] implementation
    pub fn into_inner(self) -> L {
        self.layer
    }

    pub fn handler<S>(&self, handler: S) -> L::Service
    where
        L: Layer<S>,
    {
        self.layer.layer(handler)
    }
}

impl<L: fmt::Debug> fmt::Debug for AuthenticationBuilder<L> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("AuthenticationBuilder")
            .field(&self.layer)
            .finish()
    }
}

impl<S, L> Layer<S> for AuthenticationBuilder<L>
where
    L: Layer<S>,
{
    type Service = L::Service;

    fn layer(&self, inner: S) -> Self::Service {
        self.layer.layer(inner)
    }
}
