use crate::derived_fields::DerivedField;
use darling::{FromDeriveInput, ast::Data, util::WithOriginal};

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(validate), supports(struct_any))]
pub struct DerivedStruct {
    ident: syn::Ident,
    generics: syn::Generics,
    data: Data<(), WithOriginal<DerivedField, syn::Field>>,
    // We can support context later's and other features
}
