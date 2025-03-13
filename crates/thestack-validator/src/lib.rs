#![forbid(unsafe_code)]
// Silence the noise in development!
#![cfg_attr(debug_assertions, allow(dead_code, unused_variables))]
// Docs and linting rules
#![cfg_attr(docsrs, feature(doc_auto_cfg, doc_cfg))]
#![cfg_attr(test, allow(clippy::float_cmp))]
#![cfg_attr(not(test), deny(clippy::print_stdout, clippy::dbg_macro))]
// - Lint for missing docs
#![cfg_attr(not(debug_assertions), deny(missing_docs))]
//!
//! # thestack-extractor
//!
//! Write some docs about the thestack-validator here!
mod context;
pub use context::{Error, ValidatorContext};
pub use validator::Validate;

pub trait Validator<T> {
    fn validate(&self, payload: &T) -> Result<(), Error>;
}
//
// pub type Body<T> = __private::Body<T, validator::Validate>;

// mod __private {
//     pub struct Body<T, V>
//     where
//         V: crate::Validator<T>,
//     {
//         payload: T,
//         context: V,
//     }
// }
/*

html
head
head

body
<p>{window.___Server.data.name}<p/>
body
html

*/

#[cfg(feature = "validator")]
mod validator {
    use crate::{Error, Validator};

    // Document this ot be know this allow validator crate to work
    pub struct Validate;
    impl<T> Validator<T> for Validate
    where
        T: validator::Validate,
    {
        fn validate(&self, payload: &T) -> Result<(), Error> {
            // payload.validate()

            todo!()
        }
    }
}

#[cfg(feature = "serde_valid")]
mod serde_valid {
    use crate::{Error, Validator};

    // Document this ot be know this allow validator crate to work
    pub struct SerdeValid;
    impl<T> Validator<T> for SerdeValid
    where
        T: serde_valid::Validate,
    {
        fn validate(&self, payload: &T) -> Result<(), Error> {
            // payload.validate()

            todo!()
        }
    }
}
