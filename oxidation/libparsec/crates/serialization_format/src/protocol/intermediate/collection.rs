use std::collections::HashMap;

use crate::protocol::parser;

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
                for version in variant.major_versions {
                    let entry = clc.get_or_insert_default_versioned_cmds(version);
                    entry.push(variant.clone().into())
                }
            }
        }
        clc
    }
}

pub struct Cmd {
    pub label: String,
    pub req: parser::Request,
    pub possible_response: Vec<parser::Response>,
    pub nested_types: Vec<parser::CustomType>,
}

impl From<parser::Cmd> for Cmd {
    fn from(cmd: parser::Cmd) -> Self {
        Self {
            label: cmd.label,
            req: cmd.req,
            possible_response: cmd.possible_response,
            nested_types: cmd.nested_types,
        }
    }
}
