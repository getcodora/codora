use ___private_meta::*;
use darling::{FromField, util::Override};
use proc_macro_error2::abort;
use syn::{Attribute, Field, spanned::Spanned as _};

#[derive(Debug, FromField, Clone)]
#[darling(attributes(validate))]
pub struct StructField {
    pub ident: Option<syn::Ident>,
    pub ty: syn::Type,
    //
    // A field could contain one of this
    pub skip: Option<bool>,
    #[darling(multiple)]
    pub custom: Vec<Custom>,
    pub nested: Option<bool>,
    pub regex: Option<Regex>,
    pub range: Option<Range>,
    pub length: Option<Length>,
    pub ip: Option<Override<Ip>>,
    pub url: Option<Override<Url>>,
    pub must_match: Option<MustMatch>,
    pub email: Option<Override<Email>>,
    pub credit_card: Option<Override<Card>>,
    pub required: Option<Override<Required>>,
    pub non_control_character: Option<Override<NonControlCharacter>>,
}

pub fn struct_field_validate(struct_field: &StructField, ident: &syn::Ident, fields: &[&Field], current_field: &Field) {
    let field_name = struct_field
        .ident
        .clone()
        .expect("Field is not a named field")
        .to_string();
    let field_attrs = &current_field.attrs;

    for attr in field_attrs {
        if attr.path().is_ident("validate") && matches!(attr.meta, syn::Meta::Path(_)) {
            abort!(
                current_field.span(), "You need to set at least one validator on field `{}`", field_name;
                note = "If you want nested validation, use `#[validate(nested)]`"
            )
        }
    }

    for c in &struct_field.custom {
        // If function is not a path
        if let Err(e) = &c.function {
            abort!(
                e.span(), "Invalid attribute #[validate(custom(...))] on field `{}`:", field_name;
                note = "Invalid argument for `custom` validator, only paths are allowed";
                help = "Try formating the argument like `path::to::function` or `\"path::to::function\"`"
            );
        }
    }

    if let Some(length) = &struct_field.length {
        // If length has both `equal` and `min` or `max` argument
        if length.equal.is_some() && (length.min.is_some() || length.max.is_some()) {
            abort! {
                length.equal.clone().unwrap().span(), "Invalid attribute #[validate(length(...))] on field `{}`:", field_name;
                note = "Both `equal` and `min` or `max` have been set";
                help = "Exclusively use either the `equal` or `min` and `max` attributes"
            }
        }

        // Check if validator has no arguments
        if length.equal.is_none() && length.min.is_none() && length.max.is_none() {
            abort!(
                get_attr(field_attrs, "length").unwrap(), "Invalid attribute #[validate(length(...))] on field `{}`:", field_name;
                note = "Validator `length` requires at least 1 argument";
                help = "Add the argument `equal`, `min` or `max`"
            )
        }
    }

    if let Some(must_match) = &struct_field.must_match {
        let other_field = must_match
            .other
            .get_ident()
            .expect("Cannot get ident from `other` field value")
            .to_string();

        // Check if the other field exists
        if !fields
            .iter()
            .any(|f| f.ident.clone().unwrap() == other_field)
        {
            abort!(
                must_match.other.span(), "Invalid attribute for #[validate(must_match(...))] on field `{}`:", field_name;
                note =  "The `other` field doesn't exist in the struct `{}`", ident;
                help = "Add the field `{}` to the struct", other_field
            )
        }
    }

    if let Some(range) = &struct_field.range {
        // Check if validator has no arguments
        if range.min.is_none() && range.max.is_none() && range.exclusive_min.is_none() && range.exclusive_max.is_none() {
            abort!(
                get_attr(field_attrs, "range").unwrap(),  "Invalid attribute #[validate(range(...))] on field `{}`:", field_name;
                note = "Validator `range` requires at least 1 argument";
                help = "Add the argument `min` or `max`, `exclusive_min` or `exclusive_max`"
            )
        }
    }
}
pub fn get_attr<'a>(attrs: &'a [Attribute], name: &str) -> Option<&'a Attribute> {
    attrs.iter().find(|a| match &a.meta {
        syn::Meta::List(list) => list
            .tokens
            .clone()
            .into_iter()
            .any(|t| match t {
                proc_macro2::TokenTree::Ident(i) => i == name,
                _ => false,
            }),
        _ => false,
    })
}

mod ___private_meta {
    use darling::FromMeta;
    use syn::{Expr, Path};

    #[derive(Debug, Clone, FromMeta, Default)]
    pub struct Email {
        pub message: Option<String>,
    }

    // Give email

    #[derive(Debug, Clone, FromMeta, Default)]
    pub struct Card {
        pub message: Option<String>,
    }

    #[derive(Debug, Clone, FromMeta, Default)]
    pub struct Ip {
        pub v4: Option<bool>,
        pub v6: Option<bool>,
        pub message: Option<String>,
    }

    #[derive(Debug, Clone, FromMeta)]
    pub struct Length {
        pub min: Option<Expr>,
        pub max: Option<Expr>,
        pub equal: Option<Expr>,
        pub message: Option<String>,
    }

    #[derive(Debug, Clone, FromMeta)]
    pub struct MustMatch {
        pub other: Path,
        pub message: Option<String>,
    }

    #[derive(Debug, Clone, FromMeta, Default)]
    pub struct NonControlCharacter {
        pub message: Option<String>,
    }

    #[derive(Debug, Clone, FromMeta)]
    pub struct Range {
        pub min: Option<Expr>,
        pub max: Option<Expr>,
        pub message: Option<String>,
        pub exclusive_min: Option<Expr>,
        pub exclusive_max: Option<Expr>,
    }

    #[derive(Debug, Clone, FromMeta, Default)]
    pub struct Required {
        pub message: Option<String>,
        pub code: Option<String>,
    }

    #[derive(Debug, Clone, FromMeta, Default)]
    pub struct Url {
        pub message: Option<String>,
        pub code: Option<String>,
    }

    #[derive(Debug, Clone, FromMeta)]
    pub struct Regex {
        pub path: Expr,
        pub message: Option<String>,
        pub code: Option<String>,
    }

    #[derive(Debug, Clone, FromMeta)]
    pub struct Custom {
        pub function: darling::Result<Path>,
        pub use_context: Option<bool>,
        pub message: Option<String>,
        pub code: Option<String>,
    }

    macro_rules! impl_to_tokens {
        ($ident:ident) => {};
    }

    impl_to_tokens!(Card);
    impl_to_tokens!(Email);
}
