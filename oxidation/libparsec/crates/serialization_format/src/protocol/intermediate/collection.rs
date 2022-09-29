use itertools::Itertools;
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
            for variant in protocol.0 {
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
            .sorted_by_key(|(v, _)| *v)
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
    let module_ident = module.ident.clone();
    let variant = syn::parse_quote! {
        #[serde(rename = #command_name)]
        #name(#module_ident::Req)
    };

    (variant, module)
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use super::{parser, Cmd, ProtocolCollection};

    use pretty_assertions::assert_eq;
    use proc_macro2::TokenStream;
    use quote::{quote, ToTokens};
    use rstest::rstest;

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
                parser::Protocol(vec![
                    parser::Cmd::default()
                ])
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
                parser::Protocol(vec![
                    parser::Cmd {
                        major_versions: vec![1,2,3],
                        ..Default::default()
                    }
                ])
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

    #[rstest]
    #[case::basic(
        ProtocolCollection {
            family: "foo_collection".to_string(),
            versioned_cmds: HashMap::default(),
        },
        quote! {
            pub mod foo_collection {}
        }
    )]
    #[case::with_versions(
        ProtocolCollection {
            family: "foo_collection".to_string(),
            versioned_cmds: HashMap::from([
                (2, vec![]),
                (42, vec![])
            ])
        },
        quote! {
            pub mod foo_collection {
                pub mod v2 {
                    #[derive(Debug, Clone, ::serde::Serialize, ::serde::Deserialize, PartialEq, Eq)]
                    #[serde(tag = "cmd")]
                    pub enum AnyCmdReq {}

                    impl AnyCmdReq {
                        pub fn dump(&self) -> Result<Vec<u8>, ::rmp_serde::encode::Error> {
                            ::rmp_serde::to_vec_named(self)
                        }

                        pub fn load(buf: &[u8]) -> Result<Self, ::rmp_serde::decode::Error> {
                            ::rmp_serde::from_slice(buf)
                        }
                    }
                }

                pub mod v42 {
                    #[derive(Debug, Clone, ::serde::Serialize, ::serde::Deserialize, PartialEq, Eq)]
                    #[serde(tag = "cmd")]
                    pub enum AnyCmdReq {}

                    impl AnyCmdReq {
                        pub fn dump(&self) -> Result<Vec<u8>, ::rmp_serde::encode::Error> {
                            ::rmp_serde::to_vec_named(self)
                        }

                        pub fn load(buf: &[u8]) -> Result<Self, ::rmp_serde::decode::Error> {
                            ::rmp_serde::from_slice(buf)
                        }
                    }
                }
            }
        }
    )]
    #[case::with_cmds(
        ProtocolCollection {
            family: "foo_collection".to_string(),
            versioned_cmds: HashMap::from([
                (2, vec![
                    Cmd::default(),
                    Cmd::default()
                ]),
            ])
        },
        quote! {
            pub mod foo_collection {
                pub mod v2 {
                    #[derive(Debug, Clone, ::serde::Serialize, ::serde::Deserialize, PartialEq, Eq)]
                    #[serde(tag = "cmd")]
                    pub enum AnyCmdReq {
                        #[serde(rename = "foo_cmd")]
                        FooCmd(foo_cmd::Req),
                        #[serde(rename = "foo_cmd")]
                        FooCmd(foo_cmd::Req)
                    }

                    impl AnyCmdReq {
                        pub fn dump(&self) -> Result<Vec<u8>, ::rmp_serde::encode::Error> {
                            ::rmp_serde::to_vec_named(self)
                        }

                        pub fn load(buf: &[u8]) -> Result<Self, ::rmp_serde::decode::Error> {
                            ::rmp_serde::from_slice(buf)
                        }
                    }

                    pub mod foo_cmd {
                        use super::AnyCmdReq;

                        #[derive(Debug, Clone, ::serde::Serialize, ::serde::Deserialize ,PartialEq ,Eq)]
                        pub struct Req;

                        impl Req {
                            pub fn new () -> Self { Self }
                        }

                        impl Req {
                            pub fn dump (self) -> Result<Vec<u8>, ::rmp_serde::encode::Error> {
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
                                        let data =::rmp_serde::from_slice::<UnknownStatus>(buf)?;
                                        Ok(Self::UnknownStatus {
                                            _status: data.status,
                                            reason: data.reason,
                                        })
                                    })
                            }
                        }
                    }

                    pub mod foo_cmd {
                        use super::AnyCmdReq;

                        #[derive(Debug, Clone, ::serde::Serialize, ::serde::Deserialize ,PartialEq ,Eq)]
                        pub struct Req;

                        impl Req {
                            pub fn new () -> Self { Self }
                        }

                        impl Req {
                            pub fn dump (self) -> Result<Vec<u8>, ::rmp_serde::encode::Error> {
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
                                        let data =::rmp_serde::from_slice::<UnknownStatus>(buf)?;
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
        }
    )]
    fn test_quote(#[case] collection: ProtocolCollection, #[case] expected: TokenStream) {
        assert_eq!(
            collection.quote().into_token_stream().to_string(),
            expected.to_string()
        );
    }
}
