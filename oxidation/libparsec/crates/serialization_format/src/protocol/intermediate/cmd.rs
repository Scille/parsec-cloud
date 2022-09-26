#[cfg(test)]
use pretty_assertions::assert_eq;

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
        let module = self.req.quote_name();

        syn::parse_quote! {
            pub mod #module {
                use super::AnyCmdReq;
            }
        }
    }

    pub fn quote_label(&self) -> syn::Ident {
        syn::parse_str(&self.label).expect("A valid command label")
    }
}

#[cfg(test)]
#[rstest::rstest]
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
