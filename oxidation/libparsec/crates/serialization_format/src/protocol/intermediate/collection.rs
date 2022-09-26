#[cfg(test)]
use pretty_assertions::assert_eq;
use std::collections::HashMap;

use super::Cmd;
use crate::protocol::parser;

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
                    entry.push(Cmd::from_parsed_cmd(variant.clone(), *version))
                }
            }
        }
        clc
    }
}

#[cfg(test)]
#[rstest::rstest]
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
                    parser::Cmd::default()
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
                        major_versions: vec![1,2,3],
                        ..Default::default()
                    }
                ]
            }
        ]
    },
    ProtocolCollection {
        family: "FooCollection".to_string(),
        versioned_cmds: HashMap::from([
            (1, vec![Cmd::default()]),
            (2, vec![Cmd::default()]),
            (3, vec![Cmd::default()])
        ]),
    }
)]
fn test_protocol_conversion(
    #[case] original: parser::ProtocolCollection,
    #[case] expected: ProtocolCollection,
) {
    assert_eq!(ProtocolCollection::from(original), expected);
}
