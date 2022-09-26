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

impl ProtocolCollection {
    pub fn quote(&self) -> syn::ItemMod {
        let family = self.quote_family();
        let versioned_cmds = self.quote_versioned_collection_cmds();

        syn::parse_quote! {
            pub mod #family {
                #(#versioned_cmds)*
            }
        }
    }

    fn quote_family(&self) -> syn::Ident {
        syn::parse_str(&self.family).expect("Invalid family")
    }

    fn quote_versioned_collection_cmds(&self) -> Vec<syn::ItemMod> {
        self.versioned_cmds
            .iter()
            .map(|(version, cmds)| quote_versioned_cmds(*version, cmds))
            .collect()
    }
}

fn quote_versioned_cmds(version: u32, cmds: &[Cmd]) -> syn::ItemMod {
    let versioned_cmds_mod = syn::parse_str::<syn::Ident>(&format!("v{version}"))
        .expect("Should be a valid module name");
    let (cmds_req, cmds): (Vec<syn::Variant>, Vec<syn::ItemMod>) =
        cmds.iter().map(quote_cmd).unzip();

    syn::parse_quote! {
        pub mod #versioned_cmds_mod {
            #[derive(Debug, Clone, ::serde::Serialize, ::serde::Deserialize, PartialEq, Eq)]
            #[serde(tag = "cmd")]
            pub enum AnyCmdReq {
                #(#cmds_req),*
            }

            impl AnyCmdReq {
                pub fn dump(&self) -> Result<Vec<u8>, ::rmp_serde::encode::Error> {
                    ::rmp_serde::to_vec_named(self)
                }

                pub fn load(buf: &[u8]) -> Result<Self, ::rmp_serde::decode::Error> {
                    ::rmp_serde::from_slice(buf)
                }
            }

            #(#cmds)*
        }
    }
}

fn quote_cmd(command: &Cmd) -> (syn::Variant, syn::ItemMod) {
    let name = command.quote_label();
    let command_name = &command.command_name();

    let module = command.quote();
    let variant = syn::parse_quote! {
        #[serde(rename = #command_name)]
        #name(#(module.ident)::Req)
    };

    (variant, module)
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
