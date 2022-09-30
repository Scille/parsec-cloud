// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use std::collections::HashMap;

use serde::Deserialize;

use crate::protocol::parser::field::{quote_fields, Field};

#[cfg_attr(test, derive(PartialEq, Eq))]
#[derive(Debug, Deserialize, Clone)]
pub struct Request {
    pub cmd: String,
    pub unit: Option<String>,
    pub other_fields: Vec<Field>,
}

#[cfg(test)]
impl Default for Request {
    fn default() -> Self {
        Self {
            cmd: "foo_cmd".to_string(),
            unit: None,
            other_fields: vec![],
        }
    }
}

impl Request {
    pub fn quote_name(&self) -> syn::Ident {
        syn::parse_str(&self.cmd).expect("A valid request cmd name")
    }

    pub fn quote(&self, types: &HashMap<String, String>) -> (syn::ItemStruct, syn::ItemImpl) {
        if let Some(unit) = &self.unit {
            self.quote_unit(unit)
        } else if self.other_fields.is_empty() {
            self.quote_empty()
        } else {
            self.quote_fields(types)
        }
    }

    fn quote_unit(&self, unit: &str) -> (syn::ItemStruct, syn::ItemImpl) {
        let unit = syn::parse_str::<syn::Ident>(unit).expect("A valid unit name");
        let shared_attr = Request::shared_derive();

        (
            syn::parse_quote! {
                #shared_attr
                pub struct Req(pub #unit);
            },
            syn::parse_quote! {
                impl Req {
                    pub fn new(value: #unit) -> Self {
                        Self(value)
                    }
                }
            },
        )
    }

    fn quote_empty(&self) -> (syn::ItemStruct, syn::ItemImpl) {
        let shared_attr = Request::shared_derive();

        (
            syn::parse_quote! {
                #shared_attr
                pub struct Req;
            },
            syn::parse_quote! {
                impl Req {
                    pub fn new() -> Self { Self }
                }
            },
        )
    }

    fn quote_fields(&self, types: &HashMap<String, String>) -> (syn::ItemStruct, syn::ItemImpl) {
        let shared_derive = Request::shared_derive();
        let fields = quote_fields(&self.other_fields, None, types);
        let (params, params_name): (Vec<syn::PatType>, Vec<syn::Ident>) = fields
            .iter()
            .map(|field| {
                let param_name = field
                    .ident
                    .as_ref()
                    .expect("Building a named struct, so it have a name")
                    .clone();
                let param = syn::PatType {
                    attrs: vec![],
                    pat: Box::new(syn::Pat::Ident(syn::PatIdent {
                        attrs: vec![],
                        by_ref: None,
                        mutability: None,
                        ident: param_name.clone(),
                        subpat: None,
                    })),
                    colon_token: *field
                        .colon_token
                        .as_ref()
                        .expect("Building a named struct, so it have a colon"),
                    ty: Box::new(field.ty.clone()),
                };

                (param, param_name)
            })
            .unzip();

        (
            syn::parse_quote! {
                #[::serde_with::serde_as]
                #shared_derive
                pub struct Req {
                    #(#fields),*
                }
            },
            syn::parse_quote! {
                impl Req {
                    pub fn new(#(#params),*) -> Self {
                        Self {
                            #(#params_name),*
                        }
                    }
                }
            },
        )
    }

    fn shared_derive() -> syn::Attribute {
        syn::parse_quote!(#[derive(Debug, Clone, ::serde::Serialize, ::serde::Deserialize, PartialEq, Eq)])
    }
}

#[cfg(test)]
#[rstest::rstest]
#[case::empty(
    Request::default(),
    quote::quote! {
        #[derive(Debug, Clone, ::serde::Serialize, ::serde::Deserialize, PartialEq, Eq)]
        pub struct Req;
    },
    quote::quote! {
        impl Req {
            pub fn new() -> Self { Self }
        }
    }
)]
#[case::with_unit(
    Request {
        unit: Some("String".to_string()),
        ..Default::default()
    },
    quote::quote! {
        #[derive(Debug, Clone, ::serde::Serialize, ::serde::Deserialize, PartialEq, Eq)]
        pub struct Req(pub String);
    },
    quote::quote! {
        impl Req {
            pub fn new(value: String) -> Self {
                Self(value)
            }
        }
    }
)]
#[case::with_fields(
    Request {
        other_fields: vec![
            Field::default(),
            Field::default()
        ],
        ..Default::default()
    },
    quote::quote! {
        #[::serde_with::serde_as]
        #[derive(Debug, Clone, ::serde::Serialize, ::serde::Deserialize, PartialEq, Eq)]
        pub struct Req {
            #[serde_as(as = "_")]
            pub foo_type: String,
            #[serde_as(as = "_")]
            pub foo_type: String
        }
    },
    quote::quote! {
        impl Req {
            pub fn new(foo_type: String, foo_type: String) -> Self {
                Self {
                    foo_type,
                    foo_type
                }
            }
        }
    }
)]
fn test_quote(
    #[case] request: Request,
    #[case] expected_definition: proc_macro2::TokenStream,
    #[case] expected_impl: proc_macro2::TokenStream,
) {
    use pretty_assertions::assert_eq;
    use quote::ToTokens;

    assert_eq!(
        request
            .quote(&HashMap::new())
            .0
            .into_token_stream()
            .to_string(),
        expected_definition.to_string()
    );
    assert_eq!(
        request
            .quote(&HashMap::new())
            .1
            .into_token_stream()
            .to_string(),
        expected_impl.to_string()
    )
}
