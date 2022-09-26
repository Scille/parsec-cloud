use std::collections::HashMap;

use crate::protocol::parser::{self, CustomType};

#[cfg_attr(test, derive(PartialEq, Eq))]
#[derive(Debug)]
pub struct ProtocolCollection {
    pub family: String,
    pub versioned_cmds: HashMap<u32, Vec<Cmd>>,
}

impl ProtocolCollection {
    fn new(family: String) -> Self {
        Self {
            family,
            versioned_cmds: HashMap::default(),
        }
    }
}
impl ProtocolCollection {
    fn get_or_insert_default_versioned_cmds(&mut self, version: u32) -> &mut Vec<Cmd> {
        self.versioned_cmds.entry(version).or_default()
    }
}

impl From<parser::ProtocolCollection> for ProtocolCollection {
    fn from(collection: parser::ProtocolCollection) -> Self {
        let mut clc = ProtocolCollection::new(collection.family);

        for protocol in collection.protocols {
            for variant in protocol.variants {
                for version in &variant.major_versions {
                    let entry = clc.get_or_insert_default_versioned_cmds(*version);
                    entry.push(Cmd::from(variant.clone(), *version))
                }
            }
        }
        clc
    }
}

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
    pub req: parser::Request,
    pub possible_responses: Vec<parser::Response>,
    pub nested_types: Vec<parser::CustomType>,
}

impl Cmd {
    fn from(cmd: parser::Cmd, version: u32) -> Self {
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

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use crate::protocol::parser::{Field, MajorMinorVersion};

    use super::{parser, Cmd, ProtocolCollection};

    use pretty_assertions::assert_eq;
    use rstest::rstest;

    #[rstest]
    #[case::basic(parser::Cmd::default(), Cmd::default())]
    #[case::request_with_previously_introduced_field(
        parser::Cmd {
            req: parser::Request {
                other_fields: vec![
                    Field {
                        introduced_in: Some(MajorMinorVersion { major: 0, minor: 4 }),
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
                    Field {
                        introduced_in: Some(MajorMinorVersion { major: 0, minor: 4 }),
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
                    Field {
                        introduced_in: Some(MajorMinorVersion { major: 2, minor: 4 }),
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
                        Field {
                            introduced_in: Some(MajorMinorVersion { major: 0, minor: 5 }),
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
                        Field {
                            introduced_in: Some(MajorMinorVersion { major: 0, minor: 5 }),
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
                        Field {
                            introduced_in: Some(MajorMinorVersion { major: 2, minor: 5 }),
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
                                    introduced_in: Some(MajorMinorVersion { major: 0, minor: 2 }),
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
                                    introduced_in: Some(MajorMinorVersion { major: 0, minor: 2 }),
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
                                    introduced_in: Some(MajorMinorVersion { major: 6, minor: 2 }),
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
                            introduced_in: Some(MajorMinorVersion { major: 0, minor: 1 }),
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
                            introduced_in: Some(MajorMinorVersion { major: 0, minor: 1 }),
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
                            introduced_in: Some(MajorMinorVersion { major: 3, minor: 1 }),
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
        assert_eq!(Cmd::from(original, 1), expected)
    }

    #[rstest]
    #[case::no_protocol(
        parser::ProtocolCollection {
            family: "FooCollection".to_string(),
            protocols: vec![],
        },
        ProtocolCollection {
            family: "FooCollection".to_string(),
            versioned_cmds: HashMap::default(),
        }
    )]
    #[case::no_version(
        parser::ProtocolCollection {
            family: "FooCollection".to_string(),
            protocols: vec![
                parser::Protocol {
                    variants: vec![
                        parser::Cmd {
                            label: "Foo".to_string(),
                            major_versions: vec![],
                            req: parser::Request {
                                cmd: "foo".to_string(),
                                unit: None,
                                other_fields: vec![]
                            },
                            possible_responses: vec![],
                            nested_types: vec![],
                        }
                    ]
                }
            ]
        },
        ProtocolCollection {
            family: "FooCollection".to_string(),
            versioned_cmds: HashMap::default(),
        }
    )]
    #[case::multiple_major_version(
        parser::ProtocolCollection {
            family: "FooCollection".to_string(),
            protocols: vec![
                parser::Protocol {
                    variants: vec![
                        parser::Cmd {
                            label: "Foo".to_string(),
                            major_versions: vec![],
                            req: parser::Request {
                                cmd: "foo".to_string(),
                                unit: None,
                                other_fields: vec![]
                            },
                            possible_responses: vec![],
                            nested_types: vec![],
                        }
                    ]
                }
            ]
        },
        ProtocolCollection {
            family: "FooCollection".to_string(),
            versioned_cmds: HashMap::default(),
        }
    )]
    fn test_protocol_conversion(
        #[case] original: parser::ProtocolCollection,
        #[case] expected: ProtocolCollection,
    ) {
        assert_eq!(ProtocolCollection::from(original), expected);
    }
}
