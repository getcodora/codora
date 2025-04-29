use darling::FromField;

#[derive(Debug, FromField, Clone)]
#[darling(attributes(validate))]
pub struct DerivedField {
    ident: Option<syn::Ident>,
    ty: syn::Type,
    //
    //
    #[darling(default)]
    message: Option<String>,
}

mod ___private_meta {
    use darling::FromMeta;
    use syn::{Expr, Path};

    #[derive(Debug, Clone, FromMeta, Default)]
    pub struct Card {
        pub message: Option<String>,
    }

    #[derive(Debug, Clone, FromMeta, Default)]
    pub struct Email {
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

    // #[derive(Debug, Clone, FromMeta)]
    // pub struct Custom {
    //     pub function: darling::Result<Path>,
    //     pub use_context: Option<bool>,
    //     pub message: Option<String>,
    //     pub code: Option<String>,
    // }

    // #[derive(Debug, Clone, FromMeta)]
    // pub struct Schema {
    //     pub function: Path,
    //     pub use_context: Option<bool>,
    //     pub skip_on_field_errors: Option<bool>,
    //     pub message: Option<String>,
    //     pub code: Option<String>,
    // }
}
