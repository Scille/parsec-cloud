// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use miniserde::Deserialize;
use proc_macro2::TokenStream;
use quote::quote;
use std::collections::HashMap;
use syn::{Ident, Type};

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
            Req::quote_unit(unit)
        } else if self.other_fields.is_empty() {
            Req::quote_empty()
        } else {
            self.quote_many_fields(types)
        }
    }

    fn quote_unit(unit: &str) -> TokenStream {
        let unit: Ident = syn::parse_str(unit).expect("Expected a valid name (Req)");
        quote! {
            #[derive(Debug, Clone, ::serde::Serialize, ::serde::Deserialize, PartialEq, Eq)]
            pub struct Req(pub #unit);

            impl Req {
                pub fn new(value: #unit) -> Self {
                    Self(value)
                }
            }
        }
    }

    fn quote_empty() -> TokenStream {
        quote! {
            #[derive(Debug, Clone, ::serde::Serialize, ::serde::Deserialize, PartialEq, Eq)]
            pub struct Req;

            impl Req {
                pub fn new() -> Self { Self }
            }
        }
    }

    fn quote_many_fields(&self, types: &HashMap<String, String>) -> TokenStream {
        let fields = quote_fields(&self.other_fields, Vis::Public, types);
        let args = self
            .other_fields
            .iter()
            .fold(TokenStream::new(), |args, field| {
                let name = field.quote_name().1;
                let ty = field.quote_type(types).0;

                let arg = quote! { #name: #ty };

                quote! {
                    #args
                    #arg,
                }
            });
        let params = self.other_fields.iter().fold(quote! {}, |params, field| {
            let name: Ident = field.quote_name().1;
            quote! {
                #params
                #name,
            }
        });

        quote! {
            #[::serde_with::serde_as]
            #[derive(Debug, Clone, ::serde::Serialize, ::serde::Deserialize, PartialEq, Eq)]
            pub struct Req {
                #fields
            }

            impl Req {
                pub fn new(#args) -> Self {
                    Self {
                        #params
                    }
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
            syn::parse_str(&to_pascal_case(&self.status)).expect("Expected a valid name (Rep)");
        let rename = SerdeAttr::Rename.quote(Some(&self.status));

        if let Some(unit) = &self.unit {
            let unit: Type = syn::parse_str(unit).expect("Expected a valid unit type (Rep)");
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
    discriminant_field: Option<String>,
    variants: Option<Vec<Variant>>,
    fields: Option<Vec<Field>>,
}

impl NestedType {
    pub(crate) fn quote(&self, types: &HashMap<String, String>) -> TokenStream {
        let name: Ident = syn::parse_str(&self.label).expect("Expected a valid name (NestedType)");
        match (&self.discriminant_field, &self.variants, &self.fields) {
            (None, None, Some(fields)) if fields.is_empty() => {
                quote! {
                    #[derive(Debug, Clone, ::serde::Serialize, ::serde::Deserialize, PartialEq, Eq)]
                    pub struct #name;
                }
            }
            (None, None, Some(fields)) => {
                let fields = quote_fields(fields, Vis::Public, types);
                quote! {
                    #[::serde_with::serde_as]
                    #[derive(Debug, Clone, ::serde::Serialize, ::serde::Deserialize, PartialEq, Eq)]
                    pub struct #name {
                        #fields
                    }
                }
            }
            (_, Some(variants), None) if !variants.is_empty() => {
                let serde_tag = SerdeAttr::Tag.quote(self.discriminant_field.as_ref());
                let variants = quote_variants(variants, types);
                quote! {
                    #[::serde_with::serde_as]
                    #[derive(Debug, Clone, ::serde::Serialize, ::serde::Deserialize, PartialEq, Eq)]
                    #serde_tag
                    pub enum #name {
                        #variants
                    }
                }
            }
            _ => panic!(
                "{} must contains variants[+tag] attributes (enum) or fields attribute (struct)",
                self.label
            ),
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
        let family: Ident = syn::parse_str(&self.family).expect("Expected a valid family");
        let mut cmds = quote! {};
        let mut any_cmd_req = quote! {};
        let mut types = HashMap::new();

        for cmd in &self.cmds {
            let name: Ident = syn::parse_str(&cmd.label).expect("Expected a valid name (Cmd)");
            let rename = &cmd.req.cmd;
            let module: Ident =
                syn::parse_str(&cmd.req.cmd).expect("Expected a valid module name (Cmd)");
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
                    use ::serde::de::{Deserializer, Error, IntoDeserializer};
                    use ::serde::ser::Serializer;

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
                        // `UnknownStatus` covers the case the server returns a valid message but with
                        // an unknown status value (given change in error status only cause a minor bump in API version)
                        // Note it is meaningless to serialize a `UnknownStatus` (you created the object from scratch, you know what it is for baka !)
                        #[serde(skip)]
                        UnknownStatus {
                            _status: String,
                            reason: Option<String>,
                        }
                    }

                    #[derive(::serde::Deserialize)]
                    struct UnknownStatus {
                        status: String,
                        reason: Option<String>,
                    }

                    impl Rep {
                        pub fn dump(&self) -> Result<Vec<u8>, &'static str> {
                            ::rmp_serde::to_vec_named(self).map_err(|_| "Serialization failed")
                        }

                        pub fn load(buf: &[u8]) -> Result<Self, ::rmp_serde::decode::Error> {
                            Ok(if let Ok(data) = ::rmp_serde::from_slice::<Self>(buf) {
                                data
                            } else {
                                // Due to how Serde handles variant discriminant, we cannot express unknown status as a default case in the main schema
                                // Instead we have this additional deserialization attempt fallback
                                let data = ::rmp_serde::from_slice::<UnknownStatus>(buf)?;
                                Self::UnknownStatus {
                                    _status: data.status,
                                    reason: data.reason,
                                }
                            })
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
                        ::rmp_serde::from_slice(buf).map_err(|_| "Deserialization failed")
                    }
                }

                #cmds
            }
        }
    }
}
