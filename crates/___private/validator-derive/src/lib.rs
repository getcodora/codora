#![forbid(unsafe_code)]
// Silence the noise in development!
#![cfg_attr(debug_assertions, allow(dead_code, unused_variables))]

use darling::FromDeriveInput;
use derived_struct::DerivedStruct;
use proc_macro_error2::{abort, proc_macro_error};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use struct_field::StructField;
use syn::{DeriveInput, Ident, PathArguments, parse_macro_input};

mod derived_struct;
mod struct_field;

#[proc_macro_error]
#[proc_macro_derive(Validate, attributes(validate))]
pub fn derive_validate(tk: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input: DeriveInput = parse_macro_input!(tk);

    let ds = match DerivedStruct::from_derive_input(&input) {
        Ok(ds) => ds,
        Err(e) => return e.write_errors().into(),
    };
    let derived_striuct_vis = ds.vis;
    let derived_struct_ident = ds.ident;

    let non_skip_fields = ds
        .data
        .take_struct()
        .unwrap()
        .fields
        .into_iter()
        .filter_map(|f| {
            let parsed_field = f.parsed;
            match parsed_field.skip {
                Some(s) if s => None,
                _ => Some(parsed_field),
            }
        })
        .collect::<Vec<_>>();

    #[rustfmt::skip]
    let (normal_fields, struct_fields) = non_skip_fields
        .iter()
        .fold((Option::<Vec<&StructField>>::None, Option::<Vec<&StructField>>::None), |(mut fields, mut struct_fields), field| {
            match field.nested {
                Some(n) if n => { struct_fields.get_or_insert_with(Vec::new).push(field); }
                _ => { fields.get_or_insert_with(Vec::new).push(field); }
            }

            (fields, struct_fields)
        });

    let error_struct_ident = format_ident!("{}ValidationError", derived_struct_ident);
    let struct_definition = match (normal_fields.as_ref(), struct_fields.as_ref()) {
        (None, None) => abort! {
            derived_struct_ident,
            "all struct fields can't be empty or skipped.";
            help = "Please ensure fields contain at least one non-skipped field."
        },

        _ => {
            let generic_ident = syn::Ident::new("T", proc_macro2::Span::call_site());
            let gen_token = quote! { <#generic_ident> };

            #[rustfmt::skip]
            let nf_fields = normal_fields.map_or(quote!(), |fields| { fields.iter().fold(quote! {}, |acc, f| {
                let ident = &f.ident;
                quote!(#acc #ident: #generic_ident,)
            })});

            #[rustfmt::skip]
            let sf_fields = struct_fields.map_or(quote!(), |fields| { fields.iter().fold(quote!(), |acc, f| {
                let ident = f.ident.as_ref().expect("struct field must have an identifier");
                let ty = &f.ty;

                //TODO: Support rename later and ways to make sure field_dent implement validate and it exists 
                let field_ident = format_ident!("{}ValidationError", capitalize_ident(ident));
                quote!(#acc #ident: #field_ident<#generic_ident>,)
            })});

            quote! {
                #[derive(Debug, Clone)]
                #derived_striuct_vis struct #error_struct_ident #gen_token {
                    #nf_fields
                    #sf_fields
                }
            }
        }
    };

    let (_, ty, whr) = ds.generics.split_for_impl();
    let validation_result = quote! {};

    let impl_definition = {
        let context = ds
            .context
            .as_ref()
            .map_or(quote!(()), |ctx| quote!(#ctx));

        let context_generic = ds
            .context
            .as_ref()
            .and_then(get_generic_arg_from_path)
            .unwrap_or_else(TokenStream::new);

        // TODO Support Mutable Context Later
        // let fn_args = if ds.mutable {
        //     // change this to &mut
        //     quote!(&#context)
        // } else {
        //     quote!(&#context)
        // };

        quote! {
            const _: () = {
                use codora_validator as ___codora_validator;
                use std::borrow::Cow;

                impl ___codora_validator::IntoError for #error_struct_ident<___codora_validator::ValidationError> {
                    type Output = #error_struct_ident<Option<Cow<'static, str>>>;
                    fn into_error(self) -> Self::Output {
                        #error_struct_ident<Option<Cow<'static, str>>> {
                            // create this value from field
                            // ex: value: self.ident.into(),
                        }
                    }
                }
                impl #context_generic ___codora_validator::Validate<#context> for #derived_struct_ident #ty #whr {
                    type Error = #error_struct_ident<___codora_validator::ValidationError>;

                    fn validate(&self, context: &#context) -> core::result::Result<(), Self::Error> {
                        #validation_result
                    }
                }

                // Let Genenerate Serialize and Deserialize
            };
        }
    };
    quote! {
        #struct_definition
        #impl_definition
    }
    .into()
}

fn get_generic_arg_from_path(path: &syn::Path) -> Option<TokenStream> {
    path.segments
        .iter()
        .rev()
        .find_map(|segment| {
            if let PathArguments::AngleBracketed(args) = &segment.arguments {
                Some(quote!(#args))
            } else {
                None
            }
        })
}

fn capitalize_ident(ident: &Ident) -> Ident {
    let s = ident.to_string();
    let mut c = s.chars();
    let capitalized = match c.next() {
        Some(first) => first.to_uppercase().collect::<String>() + c.as_str(),
        None => s, // fallback, shouldn't happen with a valid ident
    };
    Ident::new(&capitalized, ident.span())
}
