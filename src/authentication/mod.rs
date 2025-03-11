use async_trait::async_trait;

/*
    TODO
    * Make error generic so we can compose errror together
    * Provide a simplified api to implement `Authentication`
*/

// Replace this with actual claimmap
type ClaimMap = &'static str;

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

    /// Sign in the current request
    ///
    /// This method is called to sign-in
    ///
    /// # Arguments
    /// `state` - The current state of the request `Self::State`
    /// `claimmap` - The claimmap to be used for signin in `ClaimMap`
    async fn sign_in(&self, state: &State, claimmap: &ClaimMap) -> Result<(), Self::Error>;

    /// Sign out in the current request
    ///
    /// This method is called to sign-in
    ///
    /// # Arguments
    /// `state` - The current state of the request `Self::State`
    /// `claimmap` - The claimmap to be used for signout in `ClaimMap`
    async fn sign_out(&self, state: &State, claimmap: &ClaimMap) -> Result<(), Self::Error>;
}

#[cfg(test)]
mod test {}
