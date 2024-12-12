#![cfg_attr(
    debug_assertions,
    allow(dead_code, unused_imports, unused_variables)
)]
#[macro_use]
extern crate std_plus;
mod claims;

use async_trait::async_trait;
pub use claims::*;
use std::{borrow::Cow, collections::HashMap};

#[async_trait]
pub trait Authentication {
    type Error;

    // This is useful for error handling and logging
    const NAME: &'static str;

    /// Authenticate the current Request
    async fn authenticate(&self) -> Result<(), Self::Error>;

    //  We might need to return something that can be turn to response cause we are not mutating response
    /// Forbid the current request
    async fn forbid(&self, state: State);

    /// Challenge the current request
    async fn challenge(&self, state: State);

    /// Sign in the current Request
    async fn sign_in(&self, state: State) -> Result<(), Self::Error>;

    /// Sign out the current Request
    async fn sign_out(&self, state: State) -> Result<(), Self::Error>;
}

#[derive(Clone, Debug, Default)]
pub struct State {
    pub allow_refresh: bool,
    pub is_persistent: bool,
    pub redirect_url: Cow<'static, str>,

    // Store unnamed state
    _state: HashMap<String, String>,
    //
}

impl State {
    pub fn set_url(&mut self, url: Cow<'static, str>) -> &mut Self {
        self.redirect_url = url;
        self
    }

    //  Return Self allow chaining and setting multiple values at once
    pub fn set_state(&mut self, key: &str, value: &str) -> &mut Self {
        self._state
            .insert(key.into(), value.into());
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
}
