// #[derive(new)]
// pub struct AuthenticationService<S, T> {
//     inner: S,
//     // Other default service will be added later
//     state: State,
//     auth_extensions: Vec<T>,
// }

// pub struct AuthenticationLayer<T> {
//     // Other default service will be added later
//     state: State,
//     auth_extensions: Vec<T>,
// }

// impl<S, T, ReqBody> Service<Request<ReqBody>> for AuthenticationService<S, T>
// where
//     S: Service<Request<ReqBody>>,
//     T: Authentication<Request<ReqBody>> + Clone + Send + Sync + 'static,
// {
//     type Response = S::Response;
//     type Error = S::Error;
//     type Future = S::Future;

//     fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
//         self.inner.poll_ready(cx)
//     }

//     fn call(&mut self, mut req: Request<ReqBody>) -> Self::Future {
//         let ext = req.extensions_mut();
//         ext.insert(self.state.clone());

//         while let Some(service) = self.auth_extensions.iter().next() {
//             // T::NAME;

//             ext.insert(service.clone());
//         }

//         self.inner.call(req)
//     }
// }

// impl<S, T> Layer<S> for AuthenticationLayer<T>
// where
//     T: Clone,
// {
//     type Service = AuthenticationService<S, T>;

//     fn layer(&self, inner: S) -> Self::Service {
//         AuthenticationService::new(inner, self.state.clone(), self.auth_extensions.clone())
//     }
// }

// # [package]
// # name = "auth-rs"
// # version = "0.1.0"
// # edition = "2021"

// # [features]
// # default=["dev"]
// # dev=["oauth", "bearer", "jwt", "cookie", "axum"]
// # oauth=[]
// # bearer=[]
// # jwt=[]
// # cookie=[]
// # axum = ["dep:axum-core"]

// # [dependencies]
// # async-trait = "0.1.83"
// # axum-core = { version = "0.4.3", optional = true }
// # derive-new = "0.7.0"
// # http = "1.1.0"
// # tower-layer = "0.3.3"
// # tower-service = "0.3.3"

// # # Extenstion
// # std-plus = { git = "https://github.com/0x28west-dev/std-plus", rev = "99a17bbb1670065574eb8346f8ddfcac2dc69450" }
// # axum-plus = {git = "https://github.com/0x28west-dev/axum-plus", rev = "9247c31a9f487453d91dcf8146af03346a51c095" }

// # [dev-dependencies]
// # anyhow = "1.0.87"
// # bytes = "1.7.1"
// # futures-util = "0.3.30"
// # http-body = "1.0.1"
// # http-body-util = "0.1.2"
// # tokio = { version = "1.40.0", features = ["full"] }
// # tower = { version = "0.5.0", features = ["full"] }
