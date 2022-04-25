// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use miniserde::Deserialize;
use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

use super::utils::to_pascal_case;
use super::{quote_fields, Field, SerdeAttr, Vis};

#[derive(Deserialize)]
struct Req {
    cmd: String,
    other_fields: Vec<Field>,
}

impl Req {
    fn quote(&self) -> TokenStream {
        let fields = quote_fields(&self.other_fields, Vis::Public);
        quote! {
            #[::serde_with::serde_as]
            #[derive(Debug, Clone, ::serde::Serialize, ::serde::Deserialize, PartialEq, Eq)]
            pub struct Req {
                #fields
            }
        }
    }
}

#[derive(Deserialize)]
struct Rep {
    status: String,
    other_fields: Vec<Field>,
}

impl Rep {
    fn quote(&self) -> TokenStream {
        let name: Ident =
            syn::parse_str(&to_pascal_case(&self.status)).unwrap_or_else(|_| unreachable!());
        let rename = SerdeAttr::Rename.quote(Some(&self.status));
        let fields = quote_fields(&self.other_fields, Vis::Private);

        if self.other_fields.is_empty() {
            quote! {
                #rename
                #name
            }
        } else {
            quote! {
                #rename
                #name {
                    #fields
                }
            }
        }
    }
}

fn quote_reps(_reps: &[Rep]) -> TokenStream {
    let mut reps = quote! {};

    for rep in _reps {
        let rep = rep.quote();

        reps = quote! {
            #reps
            #rep,
        };
    }
    reps
}

#[derive(Deserialize)]
struct Cmd {
    label: String,
    req: Req,
    reps: Vec<Rep>,
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

        for cmd in &self.cmds {
            let name: Ident = syn::parse_str(&cmd.label).unwrap_or_else(|_| unreachable!());
            let rename = &cmd.req.cmd;
            let module: Ident = syn::parse_str(&cmd.req.cmd).unwrap_or_else(|_| unreachable!());
            let req = cmd.req.quote();
            let variants_rep = quote_reps(&cmd.reps);

            any_cmd_req = quote! {
                #any_cmd_req
                #[serde(rename = #rename)]
                #name(#module::Req),
            };

            cmds = quote! {
                #cmds

                pub mod #module {
                    use super::AnyCmdReq;

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
