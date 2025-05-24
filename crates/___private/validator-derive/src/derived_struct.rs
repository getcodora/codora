use super::struct_field::struct_field_validate;
use crate::struct_field::StructField;
use darling::{FromDeriveInput, ast::Data, util::WithOriginal};
use syn::Path;

#[derive(Debug, FromDeriveInput)]
#[darling(attributes(validate), supports(struct_named))]
#[darling(and_then = "validate")]
pub struct DerivedStruct {
    pub ident: syn::Ident,
    pub generics: syn::Generics,
    pub vis: syn::Visibility,
    pub data: Data<(), WithOriginal<StructField, syn::Field>>,
    pub context: Option<Path>,
    #[darling(default)]
    pub mutable: bool,
}

fn validate(derived_struct: DerivedStruct) -> darling::Result<DerivedStruct> {
    // Validate Lifetime
    // Validate all fields
    let Data::Struct(fields) = &derived_struct.data else {
        return Ok(derived_struct);
    };

    let original_fields = fields
        .fields
        .iter()
        .map(|field| &field.original)
        .collect::<Vec<_>>();

    for field in &fields.fields {
        struct_field_validate(&field.parsed, &derived_struct.ident, &original_fields, &field.original)
    }
    Ok(derived_struct)
}
