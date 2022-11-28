// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use std::collections::HashMap;

use itertools::Itertools;

use super::{CustomType, CustomTypes, Request, Responses};
use crate::{
    protocol::parser,
    shared::{filter_out_future_fields, MajorMinorVersion},
};

#[cfg_attr(test, derive(PartialEq, Eq))]
#[derive(Debug)]
pub struct Cmd {
    pub label: String,
    pub introduced_in: Option<MajorMinorVersion>,
    pub req: Request,
    pub possible_responses: Responses,
    pub nested_types: CustomTypes,
}

#[cfg(test)]
impl Default for Cmd {
    fn default() -> Self {
        Self {
            label: "FooCmd".to_string(),
            introduced_in: None,
            nested_types: CustomTypes::default(),
            possible_responses: Responses::default(),
            req: parser::Request::default(),
        }
    }
}

impl Cmd {
    pub fn from_parsed_cmd(cmd: parser::Cmd, version: u32) -> Self {
        let introduced_in = cmd
            .introduced_in
            .filter(|mm_version| mm_version.major == version);

        let possible_responses = cmd
            .possible_responses
            .into_iter()
            .map(|(name, mut response)| {
                response.fields = filter_out_future_fields!(version, response.fields);
                (name, response)
            })
            .collect();

        let mut req = cmd.req;
        req.fields = filter_out_future_fields!(version, req.fields);

        let nested_types = cmd
            .nested_types
            .into_iter()
            .map(|(name, mut custom_type)| {
                match custom_type {
                    CustomType::Enum(ref mut custom_enum) => custom_enum
                        .variants
                        .iter_mut()
                        .for_each(|(_name, variant)| {
                            variant.fields = filter_out_future_fields!(version, variant.fields)
                        }),
                    CustomType::Struct(ref mut custom_struct) => {
                        custom_struct.fields =
                            filter_out_future_fields!(version, custom_struct.fields);
                    }
                }
                (name, custom_type)
            })
            .collect();

        Self {
            label: cmd.label,
            introduced_in,
            req,
            possible_responses,
            nested_types,
        }
    }

    pub fn command_name(&self) -> &str {
        &self.req.cmd
    }
}

impl Cmd {
    pub fn quote(&self) -> syn::ItemMod {
        let name = self.quote_label();
        let module = self.req.quote_name();
        let module_attrs = self
            .introduced_in
            .map(|version| {
                let message = format!(
                    "The command `{}` was introduced in `API-{}`.",
                    self.label, version
                );
                let attr: syn::Attribute = syn::parse_quote!(#[doc = #message]);
                vec![attr]
            })
            .unwrap_or_default();
        // TODO: We may need to have the types passed as argument for subsecant custom type.
        let mut types = HashMap::new();
        self.nested_types.iter().for_each(|(name, _nested_type)| {
            types.insert(name.clone(), name.clone());
        });
        let nested_types = self.quote_nested_types(&types);
        let req = self.req.quote(&types);
        let valid_statuses = self
            .possible_responses
            .keys()
            .sorted()
            .cloned()
            .collect::<Vec<String>>();
        let mut variants_rep = self.quote_reps(&types);

        variants_rep.push(syn::parse_quote! {
            /// `UnknownStatus` covers the case the server returns a valid message but with
            /// an unknown status value (given change in error status only cause a minor bump in API version)
            /// > Note it is meaningless to serialize a `UnknownStatus` (you created the object from scratch, you know what it is for baka !)
            // TODO: Test `serde(other)`
            #[serde(skip)]
            UnknownStatus {
                unknown_status: String,
                reason: Option<String>
            }
        });

        syn::parse_quote! {
            #(#module_attrs)*
            pub mod #module {
                use super::AnyCmdReq;

                #(#nested_types)*

                #req

                impl Req {
                    pub fn dump(self) -> Result<Vec<u8>, ::rmp_serde::encode::Error> {
                        AnyCmdReq::#name(self).dump()
                    }
                }

                // Can't derive Eq because some Rep have f64 field
                #[allow(clippy::derive_partial_eq_without_eq)]
                #[::serde_with::serde_as]
                #[derive(Debug, Clone, ::serde::Serialize, ::serde::Deserialize, PartialEq)]
                #[serde(tag = "status")]
                pub enum Rep {
                    #(#variants_rep),*
                }

                #[derive(::serde::Deserialize)]
                struct UnknownStatus {
                    status: String,
                    reason: Option<String>
                }

                impl Rep {
                    pub fn dump(&self) -> Result<Vec<u8>, ::rmp_serde::encode::Error> {
                        ::rmp_serde::to_vec_named(self)
                    }

                    pub fn load(buf: &[u8]) -> Result<Self, ::rmp_serde::decode::Error> {
                        ::rmp_serde::from_slice::<Self>(buf)
                            .or_else(|err| {
                                // Due to how Serde handles variant discriminant, we cannot express unknown status as a default case in the main schema
                                // Instead we have this additional deserialization attempt fallback
                                let data = ::rmp_serde::from_slice::<UnknownStatus>(buf)?;

                                match data.status.as_str() {
                                    #(#valid_statuses => Err(err),)*
                                    _ => Ok(Self::UnknownStatus {
                                        unknown_status: data.status,
                                        reason: data.reason,
                                    })
                                }
                            })
                    }
                }
            }
        }
    }

    pub fn quote_label(&self) -> syn::Ident {
        syn::parse_str(&self.label).expect("A valid command label")
    }

    pub fn quote_nested_types(&self, types: &HashMap<String, String>) -> Vec<syn::Item> {
        self.nested_types
            .iter()
            .map(|(name, nested_type)| nested_type.quote(name, types))
            .collect()
    }

    pub fn quote_reps(&self, types: &HashMap<String, String>) -> Vec<syn::Variant> {
        self.possible_responses
            .iter()
            .sorted_by_key(|(name, _)| *name)
            .map(|(name, response)| response.quote(name, types))
            .collect()
    }
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;
    use proc_macro2::TokenStream;
    use quote::{quote, ToTokens};
    use rstest::rstest;

    use super::{parser, Cmd, Responses};

    use crate::{
        protocol::intermediate::Response,
        shared::{Field, Fields},
    };

    #[rstest]
    #[case::basic(parser::Cmd::default(), Cmd::default())]
    #[case::request_with_previously_introduced_field(
        parser::Cmd {
            req: parser::Request {
                fields: Fields::from([
                    ("foo".to_string(), Field {
                        introduced_in: Some("0.4".parse().unwrap()),
                        ..Default::default()
                    })
                ]),
                ..Default::default()
            },
            ..Default::default()
        },
        Cmd {
            req: parser::Request {
                fields: Fields::from([
                    ("foo".to_string(), Field {
                        ..Default::default()
                    })
                ]),
                ..Default::default()
            },
            ..Default::default()
        }
    )]
    #[case::request_with_not_yet_introduced_field(
        parser::Cmd {
            req: parser::Request {
                fields: Fields::from([
                    ("foo".to_string(), Field {
                        introduced_in: Some("2.4".parse().unwrap()),
                        ..Default::default()
                    })
                ]),
                ..Default::default()
            },
            ..Default::default()
        },
        Cmd {
            req: parser::Request {
                fields: Fields::default(),
                ..Default::default()
            },
            ..Default::default()
        }
    )]
    #[case::possible_response_with_introduced_field(
        parser::Cmd {
            possible_responses: Responses::from([
                ("ok".to_string(), parser::Response {
                    fields: Fields::from([
                        ("foo".to_string(), Field {
                            introduced_in: Some("0.5".parse().unwrap()),
                            ..Default::default()
                        })
                    ]),
                    ..Default::default()
                })
            ]),
            ..Default::default()
        },
        Cmd {
            possible_responses: parser::Responses::from([
                ("ok".to_string(), parser::Response {
                    fields: Fields::from([
                        ("foo".to_string(), Field {
                            ..Default::default()
                        })
                    ]),
                    ..Default::default()
                })
            ]),
            ..Default::default()
        }
    )]
    #[case::possible_response_with_not_yet_introduced_field(
        parser::Cmd {
            possible_responses: parser::Responses::from([
                ("ok".to_string(), parser::Response {
                    fields: Fields::from([
                        ("foo".to_string(), Field {
                            introduced_in: Some("2.5".parse().unwrap()),
                            ..Default::default()
                        })
                    ]),
                    ..Default::default()
                })
            ]),
            ..Default::default()
        },
        Cmd {
            possible_responses: parser::Responses::from([
                ("ok".to_string(), parser::Response {
                    fields: Fields::default(),
                    ..Default::default()
                })
            ]),
            ..Default::default()
        }
    )]
    #[case::nested_type_enum_with_introduced_field(
        parser::Cmd {
            nested_types: parser::CustomTypes::from([
                ("Foo".to_string(), parser::CustomType::Enum(parser::CustomEnum {
                    variants: parser::Variants::from([
                        ("Bar".to_string(), parser::Variant {
                            fields: Fields::from([
                                ("foo".to_string(), Field {
                                    introduced_in: Some("0.2".parse().unwrap()),
                                    ..Default::default()
                                })
                            ]),
                            ..Default::default()
                        })
                    ]),
                    ..Default::default()
                }))
            ]),
            ..Default::default()
        },
        Cmd {
            nested_types: parser::CustomTypes::from([
                ("Foo".to_string(), parser::CustomType::Enum(parser::CustomEnum {
                    variants: parser::Variants::from([
                        ("Bar".to_string(), parser::Variant {
                            fields: Fields::from([
                                ("foo".to_string(), Field {
                                    ..Default::default()
                                })
                            ]),
                            ..Default::default()
                        })
                    ]),
                    ..Default::default()
                }))
            ]),
            ..Default::default()
        }
    )]
    #[case::nested_type_enum_with_not_yet_introduced_field(
        parser::Cmd {
            nested_types: parser::CustomTypes::from([
                ("Foo".to_string(), parser::CustomType::Enum(parser::CustomEnum {
                    variants: parser::Variants::from([
                        ("Bar".to_string(), parser::Variant {
                            fields: Fields::from([
                                ("foo".to_string(), Field {
                                    introduced_in: Some("6.2".parse().unwrap()),
                                    ..Default::default()
                                })
                            ]),
                            ..Default::default()
                        })
                    ]),
                    ..Default::default()
                }))
            ]),
            ..Default::default()
        },
        Cmd {
            nested_types: parser::CustomTypes::from([
                ("Foo".to_string(), parser::CustomType::Enum(parser::CustomEnum {
                    variants: parser::Variants::from([
                        ("Bar".to_string(), parser::Variant {
                            fields: Fields::default(),
                            ..Default::default()
                        })
                    ]),
                    ..Default::default()
                }))
            ]),
            ..Default::default()
        }
    )]
    #[case::nested_type_struct_with_introduced_field(
        parser::Cmd {
            nested_types: parser::CustomTypes::from([
                ("Foo".to_string(), parser::CustomType::Struct(parser::CustomStruct {
                    fields: Fields::from([
                        ("foo".to_string(), Field {
                            introduced_in: Some("0.1".parse().unwrap()),
                            ..Default::default()
                        })
                    ])
                }))
            ]),
            ..Default::default()
        },
        Cmd {
            nested_types: parser::CustomTypes::from([
                ("Foo".to_string(), parser::CustomType::Struct(parser::CustomStruct {
                    fields: Fields::from([
                        ("foo".to_string(), Field {
                            ..Default::default()
                        })
                    ])
                }))
            ]),
            ..Default::default()
        }
    )]
    #[case::nested_type_struct_with_not_yet_introduced_field(
        parser::Cmd {
            nested_types: parser::CustomTypes::from([
                ("Foo".to_string(), parser::CustomType::Struct(parser::CustomStruct {
                    fields: Fields::from([
                        ("foo".to_string(), Field {
                            introduced_in: Some("3.1".parse().unwrap()),
                            ..Default::default()
                        })
                    ])
                }))
            ]),
            ..Default::default()
        },
        Cmd {
            nested_types: parser::CustomTypes::from([
                ("Foo".to_string(), parser::CustomType::Struct(parser::CustomStruct {
                    fields: Fields::default()
                }))
            ]),
            ..Default::default()
        }
    )]
    #[case::request_with_static_field(
        parser::Cmd {
            req: parser::Request {
                fields: Fields::from([
                    ("foo".to_string(), Field {
                        introduced_in: None,
                        ..Default::default()
                    })
                ]),
                ..Default::default()
            },
            ..Default::default()
        },
        Cmd {
            req: parser::Request {
                fields: Fields::from([
                    ("foo".to_string(), Field {
                        introduced_in: None,
                        ..Default::default()
                    })
                ]),
                ..Default::default()
            },
            ..Default::default()
        }
    )]
    #[case::possible_response_with_static_field(
        parser::Cmd {
            possible_responses: parser::Responses::from([
                ("ok".to_string(), parser::Response {
                    fields: Fields::from([
                        ("foo".to_string(), Field {
                            introduced_in: None,
                            ..Default::default()
                        })
                    ]),
                    ..Default::default()
                })
            ]),
            ..Default::default()
        },
        Cmd {
            possible_responses: parser::Responses::from([
                ("ok".to_string(), parser::Response {
                    fields: Fields::from([
                        ("foo".to_string(), Field {
                            introduced_in: None,
                            ..Default::default()
                        })
                    ]),
                    ..Default::default()
                })
            ]),
            ..Default::default()
        }
    )]
    #[case::nested_type_enum_with_static_field(
        parser::Cmd {
            nested_types: parser::CustomTypes::from([
                ("Foo".to_string(), parser::CustomType::Enum(parser::CustomEnum {
                    variants: parser::Variants::from([
                        ("Bar".to_string(), parser::Variant {
                            fields: Fields::from([
                                ("foo".to_string(), Field {
                                    introduced_in: None,
                                    ..Default::default()
                                })
                            ]),
                            ..Default::default()
                        })
                    ]),
                    ..Default::default()
                }))
            ]),
            ..Default::default()
        },
        Cmd {
            nested_types: parser::CustomTypes::from([
                ("Foo".to_string(), parser::CustomType::Enum(parser::CustomEnum {
                    variants: parser::Variants::from([
                        ("Bar".to_string(), parser::Variant {
                            fields: Fields::from([
                                ("foo".to_string(), Field {
                                    introduced_in: None,
                                    ..Default::default()
                                })
                            ]),
                            ..Default::default()
                        })
                    ]),
                    ..Default::default()
                }))
            ]),
            ..Default::default()
        }
    )]
    #[case::nested_type_struct_with_static_field(
        parser::Cmd {
            nested_types: parser::CustomTypes::from([
                ("Foo".to_string(), parser::CustomType::Struct(parser::CustomStruct {
                    fields: Fields::from([
                        ("foo".to_string(), Field {
                            introduced_in: None,
                            ..Default::default()
                        })
                    ])
                }))
            ]),
            ..Default::default()
        },
        Cmd {
            nested_types: parser::CustomTypes::from([
                ("Foo".to_string(), parser::CustomType::Struct(parser::CustomStruct {
                    fields: Fields::from([
                        ("foo".to_string(), Field {
                            introduced_in: None,
                            ..Default::default()
                        })
                    ])
                }))
            ]),
            ..Default::default()
        }
    )]
    #[case::introduced_previously(
        parser::Cmd {
            introduced_in: Some("0.4".parse().unwrap()),
            ..Default::default()
        },
        Cmd {
            introduced_in: None,
            ..Default::default()
        }
    )]
    #[case::introduced_during(
        parser::Cmd {
            introduced_in: Some("1.4".parse().unwrap()),
            ..Default::default()
        },
        Cmd {
            introduced_in: Some("1.4".parse().unwrap()),
            ..Default::default()
        }
    )]
    #[case::introduced_after(
        parser::Cmd {
            introduced_in: Some("2.4".parse().unwrap()),
            ..Default::default()
        },
        Cmd {
            introduced_in: None,
            ..Default::default()
        }
    )]
    fn test_cmd_conversion(#[case] original: parser::Cmd, #[case] expected: Cmd) {
        assert_eq!(Cmd::from_parsed_cmd(original, 1), expected)
    }

    #[rstest]
    #[case::basic(
        Cmd::default(),
        quote! {
            pub mod foo_cmd {
                use super::AnyCmdReq;

                #[derive(Debug, Clone, ::serde::Serialize, ::serde::Deserialize, PartialEq, Eq)]
                pub struct Req;

                impl Req {
                    pub fn dump(self) -> Result<Vec<u8>, ::rmp_serde::encode::Error> {
                        AnyCmdReq::FooCmd(self).dump()
                    }
                }

                #[allow(clippy::derive_partial_eq_without_eq)]
                #[::serde_with::serde_as]
                #[derive(Debug, Clone, ::serde::Serialize, ::serde::Deserialize, PartialEq)]
                #[serde(tag = "status")]
                pub enum Rep {
                    /// `UnknownStatus` covers the case the server returns a valid message but with
                    /// an unknown status value (given change in error status only cause a minor bump in API version)
                    /// > Note it is meaningless to serialize a `UnknownStatus` (you created the object from scratch, you know what it is for baka !)
                    #[serde(skip)]
                    UnknownStatus {
                        unknown_status: String,
                        reason: Option<String>
                    }
                }

                #[derive(::serde::Deserialize)]
                struct UnknownStatus {
                    status: String,
                    reason: Option<String>
                }

                impl Rep {
                    pub fn dump(&self) -> Result<Vec<u8>, ::rmp_serde::encode::Error> {
                        ::rmp_serde::to_vec_named(self)
                    }

                    pub fn load(buf: &[u8]) -> Result<Self, ::rmp_serde::decode::Error> {
                        ::rmp_serde::from_slice::<Self>(buf)
                        .or_else(|err| {
                            let data = ::rmp_serde::from_slice::<UnknownStatus>(buf)?;

                            match data.status.as_str() {
                                _ => Ok(Self::UnknownStatus {
                                    unknown_status: data.status,
                                    reason: data.reason,
                                })
                            }
                        })
                    }
                }
            }
        }
    )]
    #[case::with_possible_responses(
        Cmd {
            possible_responses: parser::Responses::from([
                ("ok".to_string(), Response::default()),
                ("err".to_string(), Response::default())
            ]),
            ..Default::default()
        },
        quote! {
            pub mod foo_cmd {
                use super::AnyCmdReq;

                #[derive(Debug, Clone, ::serde::Serialize, ::serde::Deserialize, PartialEq, Eq)]
                pub struct Req;

                impl Req {
                    pub fn dump(self) -> Result<Vec<u8>, ::rmp_serde::encode::Error> {
                        AnyCmdReq::FooCmd(self).dump()
                    }
                }

                #[allow(clippy::derive_partial_eq_without_eq)]
                #[::serde_with::serde_as]
                #[derive(Debug, Clone, ::serde::Serialize, ::serde::Deserialize, PartialEq)]
                #[serde(tag = "status")]
                pub enum Rep {
                    #[serde(rename = "err")]
                    Err,
                    #[serde(rename = "ok")]
                    Ok,
                    /// `UnknownStatus` covers the case the server returns a valid message but with
                    /// an unknown status value (given change in error status only cause a minor bump in API version)
                    /// > Note it is meaningless to serialize a `UnknownStatus` (you created the object from scratch, you know what it is for baka !)
                    #[serde(skip)]
                    UnknownStatus {
                        unknown_status: String,
                        reason: Option<String>
                    }
                }

                #[derive(::serde::Deserialize)]
                struct UnknownStatus {
                    status: String,
                    reason: Option<String>
                }

                impl Rep {
                    pub fn dump(&self) -> Result<Vec<u8>, ::rmp_serde::encode::Error> {
                        ::rmp_serde::to_vec_named(self)
                    }

                    pub fn load(buf: &[u8]) -> Result<Self, ::rmp_serde::decode::Error> {
                        ::rmp_serde::from_slice::<Self>(buf)
                        .or_else(|err| {
                            let data = ::rmp_serde::from_slice::<UnknownStatus>(buf)?;

                            match data.status.as_str() {
                                "err" => Err(err),
                                "ok" => Err(err),
                                _ => Ok(Self::UnknownStatus {
                                    unknown_status: data.status,
                                    reason: data.reason,
                                })
                            }
                        })
                    }
                }
            }
        }
    )]
    #[case::with_introduced_in(
        Cmd {
            introduced_in: Some("2.4".parse().unwrap()),
            ..Default::default()
        },
        quote! {
            #[doc = "The command `FooCmd` was introduced in `API-2.4`."]
            pub mod foo_cmd {
                use super::AnyCmdReq;

                #[derive(Debug, Clone, ::serde::Serialize, ::serde::Deserialize, PartialEq, Eq)]
                pub struct Req;

                impl Req {
                    pub fn dump(self) -> Result<Vec<u8>, ::rmp_serde::encode::Error> {
                        AnyCmdReq::FooCmd(self).dump()
                    }
                }

                #[allow(clippy::derive_partial_eq_without_eq)]
                #[::serde_with::serde_as]
                #[derive(Debug, Clone, ::serde::Serialize, ::serde::Deserialize, PartialEq)]
                #[serde(tag = "status")]
                pub enum Rep {
                    /// `UnknownStatus` covers the case the server returns a valid message but with
                    /// an unknown status value (given change in error status only cause a minor bump in API version)
                    /// > Note it is meaningless to serialize a `UnknownStatus` (you created the object from scratch, you know what it is for baka !)
                    #[serde(skip)]
                    UnknownStatus {
                        unknown_status: String,
                        reason: Option<String>
                    }
                }

                #[derive(::serde::Deserialize)]
                struct UnknownStatus {
                    status: String,
                    reason: Option<String>
                }

                impl Rep {
                    pub fn dump(&self) -> Result<Vec<u8>, ::rmp_serde::encode::Error> {
                        ::rmp_serde::to_vec_named(self)
                    }

                    pub fn load(buf: &[u8]) -> Result<Self, ::rmp_serde::decode::Error> {
                        ::rmp_serde::from_slice::<Self>(buf)
                        .or_else(|err| {
                            let data = ::rmp_serde::from_slice::<UnknownStatus>(buf)?;

                            match data.status.as_str() {
                                _ => Ok(Self::UnknownStatus {
                                    unknown_status: data.status,
                                    reason: data.reason,
                                })
                            }
                        })
                    }
                }
            }
        }
    )]
    #[case::with_introduced_in(
        Cmd {
            introduced_in: Some("2.4".parse().unwrap()),
            ..Default::default()
        },
        quote! {
            #[doc = "The command `FooCmd` was introduced in `API-2.4`."]
            pub mod foo_cmd {
                use super::AnyCmdReq;

                #[derive(Debug, Clone, ::serde::Serialize, ::serde::Deserialize, PartialEq, Eq)]
                pub struct Req;

                impl Req {
                    pub fn dump(self) -> Result<Vec<u8>, ::rmp_serde::encode::Error> {
                        AnyCmdReq::FooCmd(self).dump()
                    }
                }

                #[allow(clippy::derive_partial_eq_without_eq)]
                #[::serde_with::serde_as]
                #[derive(Debug, Clone, ::serde::Serialize, ::serde::Deserialize, PartialEq)]
                #[serde(tag = "status")]
                pub enum Rep {
                    /// `UnknownStatus` covers the case the server returns a valid message but with
                    /// an unknown status value (given change in error status only cause a minor bump in API version)
                    /// > Note it is meaningless to serialize a `UnknownStatus` (you created the object from scratch, you know what it is for baka !)
                    #[serde(skip)]
                    UnknownStatus {
                        unknown_status: String,
                        reason: Option<String>
                    }
                }

                #[derive(::serde::Deserialize)]
                struct UnknownStatus {
                    status: String,
                    reason: Option<String>
                }

                impl Rep {
                    pub fn dump(&self) -> Result<Vec<u8>, ::rmp_serde::encode::Error> {
                        ::rmp_serde::to_vec_named(self)
                    }

                    pub fn load(buf: &[u8]) -> Result<Self, ::rmp_serde::decode::Error> {
                        ::rmp_serde::from_slice::<Self>(buf)
                        .or_else(|err| {
                            let data = ::rmp_serde::from_slice::<UnknownStatus>(buf)?;

                            match data.status.as_str() {
                                _ => Ok(Self::UnknownStatus {
                                    unknown_status: data.status,
                                    reason: data.reason,
                                })
                            }
                        })
                    }
                }
            }
        }
    )]
    fn test_quote(#[case] cmd: Cmd, #[case] expected: TokenStream) {
        assert_eq!(
            cmd.quote().into_token_stream().to_string(),
            expected.to_string()
        )
    }
}
