// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use miniserde::Deserialize;
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::HashMap;
use syn::Ident;

use crate::parser::{quote_variants, Variant};

use super::utils::to_pascal_case;
use super::{quote_fields, Field, SerdeAttr, Vis};

#[derive(Deserialize)]
struct Req {
    cmd: String,
    unit: Option<String>,
    other_fields: Vec<Field>,
}

impl Req {
    fn quote(&self, types: &HashMap<String, String>) -> TokenStream {
        if let Some(unit) = &self.unit {
            let unit: Ident = syn::parse_str(unit).unwrap_or_else(|_| unreachable!());
            quote! {
                #[derive(Debug, Clone, ::serde::Serialize, ::serde::Deserialize, PartialEq, Eq)]
                pub struct Req(pub #unit);
            }
        } else if self.other_fields.is_empty() {
            quote! {
                #[derive(Debug, Clone, ::serde::Serialize, ::serde::Deserialize, PartialEq, Eq)]
                pub struct Req;
            }
        } else {
            let fields = quote_fields(&self.other_fields, Vis::Public, types);
            quote! {
                #[::serde_with::serde_as]
                #[derive(Debug, Clone, ::serde::Serialize, ::serde::Deserialize, PartialEq, Eq)]
                pub struct Req {
                    #fields
                }
            }
        }
    }
}

#[derive(Deserialize)]
struct Rep {
    status: String,
    unit: Option<String>,
    other_fields: Vec<Field>,
}

impl Rep {
    fn quote(&self, types: &HashMap<String, String>) -> TokenStream {
        let name: Ident =
            syn::parse_str(&to_pascal_case(&self.status)).unwrap_or_else(|_| unreachable!());
        let rename = SerdeAttr::Rename.quote(Some(&self.status));

        if let Some(unit) = &self.unit {
            let unit: Ident = syn::parse_str(unit).unwrap_or_else(|_| unreachable!());
            quote! {
                #rename
                #name (#unit)
            }
        } else if self.other_fields.is_empty() {
            quote! {
                #rename
                #name
            }
        } else {
            let fields = quote_fields(&self.other_fields, Vis::Private, types);
            quote! {
                #rename
                #name {
                    #fields
                }
            }
        }
    }
}

fn quote_reps(_reps: &[Rep], types: &HashMap<String, String>) -> TokenStream {
    let mut reps = quote! {};

    for rep in _reps {
        let rep = rep.quote(types);

        reps = quote! {
            #reps
            #rep,
        };
    }
    reps
}

#[derive(Deserialize)]
struct NestedType {
    label: String,
    tag: Option<String>,
    variants: Vec<Variant>,
}

impl NestedType {
    pub(crate) fn quote(&self, types: &HashMap<String, String>) -> TokenStream {
        let name: Ident = syn::parse_str(&self.label).unwrap_or_else(|_| unreachable!());
        match self.variants.len() {
            0 => {
                quote! {
                    #[derive(Debug, Clone, ::serde::Serialize, ::serde::Deserialize, PartialEq, Eq)]
                    pub struct #name;
                }
            }
            1 => {
                if self.variants[0].fields.is_empty() {
                    panic!(
                        r#"Use:  {{ "label": "{}", "variants": [] }} instead"#,
                        self.label
                    )
                }
                let fields = quote_fields(&self.variants[0].fields, Vis::Public, types);
                quote! {
                    #[::serde_with::serde_as]
                    #[derive(Debug, Clone, ::serde::Serialize, ::serde::Deserialize, PartialEq, Eq)]
                    pub struct #name {
                        #fields
                    }
                }
            }
            _ => {
                let serde_tag = SerdeAttr::Tag.quote(self.tag.as_ref());
                let variants = quote_variants(&self.variants, types);
                quote! {
                    #[::serde_with::serde_as]
                    #[derive(Debug, Clone, ::serde::Serialize, ::serde::Deserialize, PartialEq, Eq)]
                    #serde_tag
                    pub enum #name {
                        #variants
                    }
                }
            }
        }
    }
}

fn quote_nested_types(
    _nested_types: &[NestedType],
    types: &HashMap<String, String>,
) -> TokenStream {
    let mut nested_types = quote! {};

    for nested_type in _nested_types {
        let nested_type = nested_type.quote(types);
        nested_types = quote! {
            #nested_types
            #nested_type
        };
    }
    nested_types
}

#[derive(Deserialize)]
struct Cmd {
    label: String,
    req: Req,
    reps: Vec<Rep>,
    nested_types: Option<Vec<NestedType>>,
}

#[derive(Deserialize)]
pub(crate) struct Cmds {
    family: String,
    cmds: Vec<Cmd>,
}

impl Cmds {
    pub(crate) fn quote(&self) -> TokenStream {
        let family: Ident = syn::parse_str(&self.family).unwrap_or_else(|_| unreachable!());
        let mut cmds = quote! {};
        let mut any_cmd_req = quote! {};
        let mut types = HashMap::new();

        for cmd in &self.cmds {
            let name: Ident = syn::parse_str(&cmd.label).unwrap_or_else(|_| unreachable!());
            let rename = &cmd.req.cmd;
            let module: Ident = syn::parse_str(&cmd.req.cmd).unwrap_or_else(|_| unreachable!());
            let nested_types = match &cmd.nested_types {
                Some(nested_types) => {
                    // Insert local types
                    nested_types.iter().for_each(|nested_type| {
                        let ty = nested_type.label.clone();
                        types.insert(ty.clone(), ty);
                    });
                    quote_nested_types(nested_types, &types)
                }
                None => quote! {},
            };
            let req = cmd.req.quote(&types);
            let variants_rep = quote_reps(&cmd.reps, &types);

            if let Some(nested_types) = &cmd.nested_types {
                // Drop local types
                nested_types.iter().for_each(|nested_type| {
                    types.remove(&nested_type.label);
                });
            }

            any_cmd_req = quote! {
                #any_cmd_req
                #[serde(rename = #rename)]
                #name(#module::Req),
            };

            cmds = quote! {
                #cmds

                pub mod #module {
                    use super::AnyCmdReq;

                    #nested_types

                    #req

                    impl Req {
                        pub fn dump(self) -> Result<Vec<u8>, &'static str> {
                            ::rmp_serde::to_vec_named(&AnyCmdReq::#name(self))
                                .map_err(|_| "Serialization failed")
                        }
                    }

                    #[::serde_with::serde_as]
                    #[derive(Debug, Clone, ::serde::Serialize, ::serde::Deserialize, PartialEq, Eq)]
                    #[serde(tag = "status")]
                    pub enum Rep {
                        #variants_rep
                        UnknownError {
                            error: String,
                        },
                    }

                    impl Rep {
                        pub fn dump(&self) -> Result<Vec<u8>, &'static str> {
                            ::rmp_serde::to_vec_named(self).map_err(|_| "Serialization failed")
                        }

                        pub fn load(buf: &[u8]) -> Self {
                            match ::rmp_serde::from_read_ref(buf) {
                                Ok(res) => res,
                                Err(e) => Self::UnknownError {
                                    error: e.to_string(),
                                },
                            }
                        }
                    }
                }
            };
        }

        quote! {
            pub mod #family {
                #[derive(Debug, Clone, ::serde::Serialize, ::serde::Deserialize, PartialEq, Eq)]
                #[serde(tag = "cmd")]
                pub enum AnyCmdReq {
                    #any_cmd_req
                }

                impl AnyCmdReq {
                    pub fn dump(&self) -> Result<Vec<u8>, &'static str> {
                        ::rmp_serde::to_vec_named(self).map_err(|_| "Serialization failed")
                    }

                    pub fn load(buf: &[u8]) -> Result<Self, &'static str> {
                        ::rmp_serde::from_read_ref(buf).map_err(|_| "Deserialization failed")
                    }
                }

                #cmds
            }
        }
    }
}
