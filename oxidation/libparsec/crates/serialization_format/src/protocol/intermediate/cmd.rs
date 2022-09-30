// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use std::collections::HashMap;

use super::{CustomType, Request, Response};
use crate::protocol::parser;

macro_rules! filter_out_future_fields {
    ($current_version:expr, $fields:expr) => {
        $fields
            .iter()
            .filter(move |field| {
                // We check if the current field need to be present at the `current_version`
                field
                    .introduced_in
                    .map(|mj_version| mj_version.major <= $current_version)
                    .unwrap_or(true)
            })
            .cloned()
            .collect()
    };
}
#[cfg_attr(test, derive(PartialEq, Eq))]
#[derive(Debug)]
pub struct Cmd {
    pub label: String,
    pub req: Request,
    pub possible_responses: Vec<Response>,
    pub nested_types: Vec<CustomType>,
}

#[cfg(test)]
impl Default for Cmd {
    fn default() -> Self {
        Self {
            label: "FooCmd".to_string(),
            nested_types: vec![],
            possible_responses: vec![],
            req: parser::Request::default(),
        }
    }
}

impl Cmd {
    pub fn from_parsed_cmd(cmd: parser::Cmd, version: u32) -> Self {
        let possible_responses = cmd
            .possible_responses
            .into_iter()
            .map(|mut response| {
                response.other_fields = filter_out_future_fields!(version, response.other_fields);
                response
            })
            .collect();

        let mut req = cmd.req;
        req.other_fields = filter_out_future_fields!(version, req.other_fields);

        let nested_types = cmd
            .nested_types
            .into_iter()
            .map(|mut custom_type| {
                match custom_type {
                    CustomType::Enum(ref mut custom_enum) => {
                        custom_enum.variants.iter_mut().for_each(|variant| {
                            variant.fields = filter_out_future_fields!(version, variant.fields)
                        })
                    }
                    CustomType::Struct(ref mut custom_struct) => {
                        custom_struct.fields =
                            filter_out_future_fields!(version, custom_struct.fields);
                    }
                }
                custom_type
            })
            .collect();

        Self {
            label: cmd.label,
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
        // TODO: We may need to have the types passed as argument for subsecant custom type.
        let mut types = HashMap::new();
        self.nested_types.iter().for_each(|nested_type| {
            let ty = nested_type.label().to_string();
            types.insert(ty.clone(), ty);
        });
        let nested_types = self.quote_nested_types(&types);
        let (req_def, req_impl) = self.req.quote(&types);
        let mut variants_rep = self.quote_reps(&types);

        variants_rep.push(syn::parse_quote! {
            /// `UnknownStatus` covers the case the server returns a valid message but with
            /// an unknown status value (given change in error status only cause a minor bump in API version)
            /// > Note it is meaningless to serialize a `UnknownStatus` (you created the object from scratch, you know what it is for baka !)
            // TODO: Test `serde(other)`
            #[serde(skip)]
            UnknownStatus {
                _status: String,
                reason: Option<String>
            }
        });

        syn::parse_quote! {
            pub mod #module {
                use super::AnyCmdReq;

                #(#nested_types)*

                #req_def

                #req_impl

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
                            .or_else(|_error| {
                                // Due to how Serde handles variant discriminant, we cannot express unknown status as a default case in the main schema
                                // Instead we have this additional deserialization attempt fallback
                                let data = ::rmp_serde::from_slice::<UnknownStatus>(buf)?;
                                Ok(Self::UnknownStatus {
                                    _status: data.status,
                                    reason: data.reason,
                                })
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
            .map(|nested_type| nested_type.quote(types))
            .collect()
    }

    pub fn quote_reps(&self, types: &HashMap<String, String>) -> Vec<syn::Variant> {
        self.possible_responses
            .iter()
            .map(|response| response.quote(types))
            .collect()
    }
}

#[cfg(test)]
mod test {
    use pretty_assertions::assert_eq;
    use proc_macro2::TokenStream;
    use quote::{quote, ToTokens};
    use rstest::rstest;

    use super::{parser, Cmd, Response};

    #[rstest]
    #[case::basic(parser::Cmd::default(), Cmd::default())]
    #[case::request_with_previously_introduced_field(
        parser::Cmd {
            req: parser::Request {
                other_fields: vec![
                    parser::Field {
                        introduced_in: Some("0.4".try_into().unwrap()),
                        ..Default::default()
                    }
                ],
                ..Default::default()
            },
            ..Default::default()
        },
        Cmd {
            req: parser::Request {
                other_fields: vec![
                    parser::Field {
                        introduced_in: Some("0.4".try_into().unwrap()),
                        ..Default::default()
                    }
                ],
                ..Default::default()
            },
            ..Default::default()
        }
    )]
    #[case::request_with_not_yet_introduced_field(
        parser::Cmd {
            req: parser::Request {
                other_fields: vec![
                    parser::Field {
                        introduced_in: Some("2.4".try_into().unwrap()),
                        ..Default::default()
                    }
                ],
                ..Default::default()
            },
            ..Default::default()
        },
        Cmd {
            req: parser::Request {
                other_fields: vec![],
                ..Default::default()
            },
            ..Default::default()
        }
    )]
    #[case::possible_response_with_introduced_field(
        parser::Cmd {
            possible_responses: vec![
                parser::Response {
                    other_fields: vec![
                        parser::Field {
                            introduced_in: Some("0.5".try_into().unwrap()),
                            ..Default::default()
                        }
                    ],
                    ..Default::default()
                }
            ],
            ..Default::default()
        },
        Cmd {
            possible_responses: vec![
                parser::Response {
                    other_fields: vec![
                        parser::Field {
                            introduced_in: Some("0.5".try_into().unwrap()),
                            ..Default::default()
                        }
                    ],
                    ..Default::default()
                }
            ],
            ..Default::default()
        }
    )]
    #[case::possible_response_with_not_yet_introduced_field(
        parser::Cmd {
            possible_responses: vec![
                parser::Response {
                    other_fields: vec![
                        parser::Field {
                            introduced_in: Some("2.5".try_into().unwrap()),
                            ..Default::default()
                        }
                    ],
                    ..Default::default()
                }
            ],
            ..Default::default()
        },
        Cmd {
            possible_responses: vec![
                parser::Response {
                    other_fields: vec![],
                    ..Default::default()
                }
            ],
            ..Default::default()
        }
    )]
    #[case::nested_type_enum_with_introduced_field(
        parser::Cmd {
            nested_types: vec![
                parser::CustomType::Enum(parser::CustomEnum {
                    variants: vec![
                        parser::Variant {
                            fields: vec![
                                parser::Field {
                                    introduced_in: Some("0.2".try_into().unwrap()),
                                    ..Default::default()
                                }
                            ],
                            ..Default::default()
                        }
                    ],
                    ..Default::default()
                })
            ],
            ..Default::default()
        },
        Cmd {
            nested_types: vec![
                parser::CustomType::Enum(parser::CustomEnum {
                    variants: vec![
                        parser::Variant {
                            fields: vec![
                                parser::Field {
                                    introduced_in: Some("0.2".try_into().unwrap()),
                                    ..Default::default()
                                }
                            ],
                            ..Default::default()
                        }
                    ],
                    ..Default::default()
                })
            ],
            ..Default::default()
        }
    )]
    #[case::nested_type_enum_with_not_yet_introduced_field(
        parser::Cmd {
            nested_types: vec![
                parser::CustomType::Enum(parser::CustomEnum {
                    variants: vec![
                        parser::Variant {
                            fields: vec![
                                parser::Field {
                                    introduced_in: Some("6.2".try_into().unwrap()),
                                    ..Default::default()
                                }
                            ],
                            ..Default::default()
                        }
                    ],
                    ..Default::default()
                })
            ],
            ..Default::default()
        },
        Cmd {
            nested_types: vec![
                parser::CustomType::Enum(parser::CustomEnum {
                    variants: vec![
                        parser::Variant {
                            fields: vec![],
                            ..Default::default()
                        }
                    ],
                    ..Default::default()
                })
            ],
            ..Default::default()
        }
    )]
    #[case::nested_type_struct_with_introduced_field(
        parser::Cmd {
            nested_types: vec![
                parser::CustomType::Struct(parser::CustomStruct {
                    label: "Data".to_string(),
                    fields: vec![
                        parser::Field {
                            introduced_in: Some("0.1".try_into().unwrap()),
                            ..Default::default()
                        }
                    ]
                })
            ],
            ..Default::default()
        },
        Cmd {
            nested_types: vec![
                parser::CustomType::Struct(parser::CustomStruct {
                    label: "Data".to_string(),
                    fields: vec![
                        parser::Field {
                            introduced_in: Some("0.1".try_into().unwrap()),
                            ..Default::default()
                        }
                    ]
                })
            ],
            ..Default::default()
        }
    )]
    #[case::nested_type_struct_with_not_yet_introduced_field(
        parser::Cmd {
            nested_types: vec![
                parser::CustomType::Struct(parser::CustomStruct {
                    label: "Data".to_string(),
                    fields: vec![
                        parser::Field {
                            introduced_in: Some("3.1".try_into().unwrap()),
                            ..Default::default()
                        }
                    ]
                })
            ],
            ..Default::default()
        },
        Cmd {
            nested_types: vec![
                parser::CustomType::Struct(parser::CustomStruct {
                    label: "Data".to_string(),
                    fields: vec![]
                })
            ],
            ..Default::default()
        }
    )]
    #[case::request_with_static_field(
        parser::Cmd {
            req: parser::Request {
                other_fields: vec![
                    parser::Field {
                        introduced_in: None,
                        ..Default::default()
                    }
                ],
                ..Default::default()
            },
            ..Default::default()
        },
        Cmd {
            req: parser::Request {
                other_fields: vec![
                    parser::Field {
                        introduced_in: None,
                        ..Default::default()
                    }
                ],
                ..Default::default()
            },
            ..Default::default()
        }
    )]
    #[case::possible_response_with_static_field(
        parser::Cmd {
            possible_responses: vec![
                parser::Response {
                    other_fields: vec![
                        parser::Field {
                            introduced_in: None,
                            ..Default::default()
                        }
                    ],
                    ..Default::default()
                }
            ],
            ..Default::default()
        },
        Cmd {
            possible_responses: vec![
                parser::Response {
                    other_fields: vec![
                        parser::Field {
                            introduced_in: None,
                            ..Default::default()
                        }
                    ],
                    ..Default::default()
                }
            ],
            ..Default::default()
        }
    )]
    #[case::nested_type_enum_with_static_field(
        parser::Cmd {
            nested_types: vec![
                parser::CustomType::Enum(parser::CustomEnum {
                    variants: vec![
                        parser::Variant {
                            fields: vec![
                                parser::Field {
                                    introduced_in: None,
                                    ..Default::default()
                                }
                            ],
                            ..Default::default()
                        }
                    ],
                    ..Default::default()
                })
            ],
            ..Default::default()
        },
        Cmd {
            nested_types: vec![
                parser::CustomType::Enum(parser::CustomEnum {
                    variants: vec![
                        parser::Variant {
                            fields: vec![
                                parser::Field {
                                    introduced_in: None,
                                    ..Default::default()
                                }
                            ],
                            ..Default::default()
                        }
                    ],
                    ..Default::default()
                })
            ],
            ..Default::default()
        }
    )]
    #[case::nested_type_struct_with_static_field(
        parser::Cmd {
            nested_types: vec![
                parser::CustomType::Struct(parser::CustomStruct {
                    label: "Struct".to_string(),
                    fields: vec![
                        parser::Field {
                            introduced_in: None,
                            ..Default::default()
                        }
                    ]
                })
            ],
            ..Default::default()
        },
        Cmd {
            nested_types: vec![
                parser::CustomType::Struct(parser::CustomStruct {
                    label: "Struct".to_string(),
                    fields: vec![
                        parser::Field {
                            introduced_in: None,
                            ..Default::default()
                        }
                    ]
                })
            ],
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
                    pub fn new() -> Self { Self }
                }

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
                        _status: String,
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
                        .or_else(|_error| {
                            let data = ::rmp_serde::from_slice::<UnknownStatus>(buf)?;
                            Ok(Self::UnknownStatus {
                                _status: data.status,
                                reason: data.reason,
                            })
                        })
                    }
                }
            }
        }
    )]
    #[case::with_possible_responses(
        Cmd {
            possible_responses: vec![
                Response::default(),
                Response::default()
            ],
            ..Default::default()
        },
        quote! {
            pub mod foo_cmd {
                use super::AnyCmdReq;

                #[derive(Debug, Clone, ::serde::Serialize, ::serde::Deserialize, PartialEq, Eq)]
                pub struct Req;

                impl Req {
                    pub fn new() -> Self { Self }
                }

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
                    #[serde(rename = "foo_response")]
                    FooResponse,
                    #[serde(rename = "foo_response")]
                    FooResponse,
                    /// `UnknownStatus` covers the case the server returns a valid message but with
                    /// an unknown status value (given change in error status only cause a minor bump in API version)
                    /// > Note it is meaningless to serialize a `UnknownStatus` (you created the object from scratch, you know what it is for baka !)
                    #[serde(skip)]
                    UnknownStatus {
                        _status: String,
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
                        .or_else(|_error| {
                            let data = ::rmp_serde::from_slice::<UnknownStatus>(buf)?;
                            Ok(Self::UnknownStatus {
                                _status: data.status,
                                reason: data.reason,
                            })
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
