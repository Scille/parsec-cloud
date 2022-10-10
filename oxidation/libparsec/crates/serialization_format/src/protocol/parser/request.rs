// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use std::collections::HashMap;

use serde::Deserialize;

use crate::protocol::parser::field::{quote_fields, Fields};

#[cfg_attr(test, derive(PartialEq, Eq))]
#[derive(Debug, Deserialize, Clone)]
pub struct Request {
    pub cmd: String,
    pub unit: Option<String>,
    #[serde(default)]
    pub fields: Fields,
}

#[cfg(test)]
impl Default for Request {
    fn default() -> Self {
        Self {
            cmd: "foo_cmd".to_string(),
            unit: None,
            fields: Fields::default(),
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
        } else if self.fields.is_empty() {
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
        let fields = quote_fields(&self.fields, None, types);
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
mod test {
    use pretty_assertions::assert_eq;
    use quote::{quote, ToTokens};
    use rstest::rstest;

    use super::{Fields, HashMap, Request};

    use crate::protocol::parser::Field;

    #[rstest]
    #[case::unit(
        r#"{"cmd": "foo", "unit": "u32"}"#,
        Request { cmd: "foo".to_string(), unit: Some("u32".to_string()), fields: Fields::default() }
    )]
    #[case::fields(
        r#"{
            "cmd": "foo",
            "fields": {
                "foo": {
                    "type": "usize"
                }
            }
        }"#,
        Request {
            cmd: "foo".to_string(),
            unit: None,
            fields: Fields::from([
                ("foo".to_string(), Field {
                    ty: "usize".to_string(),
                    ..Default::default()
                })
            ])
        }
    )]
    fn deserialization(#[case] input: &str, #[case] expected: Request) {
        let request: Request = serde_json::from_str(input).expect("Valid json input");
        assert_eq!(request, expected);
    }

    #[rstest]
    #[case::empty(
        Request::default(),
        quote! {
            #[derive(Debug, Clone, ::serde::Serialize, ::serde::Deserialize, PartialEq, Eq)]
            pub struct Req;
        },
        quote! {
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
        quote! {
            #[derive(Debug, Clone, ::serde::Serialize, ::serde::Deserialize, PartialEq, Eq)]
            pub struct Req(pub String);
        },
        quote! {
            impl Req {
                pub fn new(value: String) -> Self {
                    Self(value)
                }
            }
        }
    )]
    #[case::with_fields(
        Request {
            fields: Fields::from([
                ("foo".to_string(), Field::default()),
                ("bar".to_string(), Field::default())
            ]),
            ..Default::default()
        },
        quote! {
            #[::serde_with::serde_as]
            #[derive(Debug, Clone, ::serde::Serialize, ::serde::Deserialize, PartialEq, Eq)]
            pub struct Req {
                #[serde_as(as = "_")]
                pub bar: String,
                #[serde_as(as = "_")]
                pub foo: String
            }
        },
        quote! {
            impl Req {
                pub fn new(bar: String, foo: String) -> Self {
                    Self {
                        bar,
                        foo
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
}
