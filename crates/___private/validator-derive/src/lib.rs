#![forbid(unsafe_code)]
// Silence the noise in development!
#![cfg_attr(debug_assertions, allow(dead_code, unused_variables))]

use darling::FromDeriveInput;
use derived_struct::DerivedStruct;
use proc_macro::TokenStream;
use proc_macro_error2::proc_macro_error;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

mod derived_fields;
mod derived_struct;

#[proc_macro_error]
#[proc_macro_derive(Validate, attributes(validate))]
pub fn derive_validate(tk: TokenStream) -> TokenStream {
    let input: DeriveInput = parse_macro_input!(tk);

    let ds = match DerivedStruct::from_derive_input(&input) {
        Ok(ds) => ds,
        Err(e) => return e.write_errors().into(),
    };

    // eprintln!("{:#?}", ds);
    quote! {}.into()
}
