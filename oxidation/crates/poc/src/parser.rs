// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use miniserde::Deserialize;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{GenericArgument, Ident, PathArguments, Type};

fn to_snake_case(s: &str) -> String {
    let mut out = s[..1].to_lowercase();
    s.chars().skip(1).for_each(|c| match c {
        c @ 'A'..='Z' => {
            out.push('_');
            out.push((c as u8 - b'A' + b'a') as char)
        }
        c @ '0'..='9' => {
            out.push('_');
            out.push(c)
        }
        c => out.push(c),
    });

    out
}

// Extract the type recursively by changing all unit type except u8 by _
fn extract_serde_as(ty: &Type) -> String {
    match ty {
        Type::Path(p) => {
            let ty = p.path.segments.last().unwrap_or_else(|| unreachable!());
            let mut ident = ty.ident.to_string();
            match &ty.arguments {
                PathArguments::None if ident == "u8" => ident,
                PathArguments::None => "_".to_string(),
                PathArguments::AngleBracketed(x) => {
                    ident.push('<');
                    ident += &x
                        .args
                        .iter()
                        .map(|arg| match arg {
                            GenericArgument::Type(ty) => extract_serde_as(ty),
                            _ => unimplemented!(),
                        })
                        .collect::<Vec<_>>()
                        .join(",");
                    ident.push('>');
                    ident
                }
                _ => unimplemented!(),
            }
        }
        Type::Tuple(t) => {
            let mut ident = String::new();
            ident.push('(');
            ident += &t
                .elems
                .iter()
                .map(extract_serde_as)
                .collect::<Vec<_>>()
                .join(",");
            ident.push(')');
            ident
        }
        ty => panic!("{ty:?} encountered"),
    }
}

fn quote_serde_as(ty: &Type) -> TokenStream {
    let serde_as = extract_serde_as(ty)
        .replace("Vec<u8>", "::serde_with::Bytes")
        .replace("u8", "_");
    quote! { #[serde_as(as = #serde_as)] }
}

enum SerdeAttr {
    Rename,
    RenameAll,
    Tag,
}

fn quote_serde_attr(attr: SerdeAttr, value: &Option<String>) -> TokenStream {
    match value {
        Some(value) => match attr {
            SerdeAttr::Rename => quote! { #[serde(rename = #value)] },
            SerdeAttr::RenameAll => quote! { #[serde(rename_all = #value)] },
            SerdeAttr::Tag => quote! { #[serde(tag = #value)] },
        },
        None => quote! {},
    }
}

#[derive(Clone, Copy)]
enum Vis {
    Public,
    Private,
}

#[derive(Deserialize)]
struct Field {
    name: String,
    #[serde(rename = "type")]
    ty: String,
    rename: Option<String>,
}

impl Field {
    fn quote(&self, vis: Vis) -> TokenStream {
        let name: Ident = syn::parse_str(&self.name).unwrap_or_else(|_| unreachable!());
        let ty: Type = syn::parse_str(&self.ty).unwrap_or_else(|e| panic!("{e}"));
        let rename = quote_serde_attr(SerdeAttr::Rename, &self.rename);
        let serde_as = quote_serde_as(&ty);

        match vis {
            Vis::Public => quote! {
                #rename
                #serde_as
                pub #name: #ty
            },
            Vis::Private => quote! {
                #rename
                #serde_as
                #name: #ty
            },
        }
    }
}

fn quote_fields(_fields: &[Field], vis: Vis) -> TokenStream {
    let mut fields = quote! {};

    for field in _fields {
        let field = field.quote(vis);
        fields = quote! {
            #fields
            #field,
        };
    }
    fields
}

#[derive(Deserialize)]
struct Variant {
    name: String,
    rename: Option<String>,
    fields: Vec<Field>,
}

impl Variant {
    fn quote(&self) -> TokenStream {
        let name: Ident = syn::parse_str(&self.name).unwrap_or_else(|_| unreachable!());
        let rename = quote_serde_attr(SerdeAttr::Rename, &self.rename);
        let fields = quote_fields(&self.fields, Vis::Private);

        if self.fields.is_empty() {
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

fn quote_variants(_variants: &[Variant]) -> TokenStream {
    let mut variants = quote! {};

    for variant in _variants {
        let variant = variant.quote();

        variants = quote! {
            #variants
            #variant,
        };
    }
    variants
}

#[derive(Deserialize)]
pub(crate) struct Schema {
    name: String,
    rename_all: Option<String>,
    // Struct based
    fields: Option<Vec<Field>>,
    // Enum based
    tag: Option<String>,
    variants: Option<Vec<Variant>>,
}

impl Schema {
    pub(crate) fn quote(&self) -> TokenStream {
        let name: Ident = syn::parse_str(&self.name).unwrap_or_else(|_| unreachable!());
        let rename_all = quote_serde_attr(SerdeAttr::RenameAll, &self.rename_all);
        let tag = quote_serde_attr(SerdeAttr::Tag, &self.tag);

        if let Some(variants) = &self.variants {
            let variants = quote_variants(variants);

            quote! {
                #[::serde_with::serde_as]
                #[derive(Debug, Clone, ::serde::Serialize, ::serde::Deserialize, PartialEq, Eq)]
                #rename_all
                #tag
                pub enum #name {
                    #variants
                }
            }
        } else if let Some(fields) = &self.fields {
            let fields = quote_fields(fields, Vis::Public);

            quote! {
                #[::serde_with::serde_as]
                #[derive(Debug, Clone, ::serde::Serialize, ::serde::Deserialize, PartialEq, Eq)]
                #rename_all
                pub struct #name {
                    #fields
                }
            }
        } else {
            quote! {
                #[derive(Debug, Clone, ::serde::Serialize, ::serde::Deserialize, PartialEq, Eq)]
                pub struct #name;
            }
        }
    }
}

#[derive(Deserialize)]
struct Cmd {
    name: String,
    rename: Option<String>,
    req: Vec<Field>,
    reps: Vec<Variant>,
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
            let name: Ident = syn::parse_str(&cmd.name).unwrap_or_else(|_| unreachable!());
            let module: Ident =
                syn::parse_str(&to_snake_case(&cmd.name)).unwrap_or_else(|_| unreachable!());
            let rename = quote_serde_attr(SerdeAttr::Rename, &cmd.rename);
            let fields_req = quote_fields(&cmd.req, Vis::Public);
            let variants_rep = quote_variants(&cmd.reps);

            any_cmd_req = quote! {
                #any_cmd_req
                #rename
                #name(#module::Req),
            };

            cmds = quote! {
                #cmds

                pub mod #module {
                    use parsec_api_types::*;
                    use super::AnyCmdReq;

                    #[::serde_with::serde_as]
                    #[derive(Debug, Clone, ::serde::Serialize, ::serde::Deserialize, PartialEq, Eq)]
                    pub struct Req {
                        #fields_req
                    }

                    impl Req {
                        pub fn dump(self) -> Result<Vec<u8>, &'static str> {
                            ::rmp_serde::to_vec_named(&AnyCmdReq::#name(self))
                                .map_err(|_| "Serialization failed")
                        }
                    }

                    #[::serde_with::serde_as]
                    #[derive(Debug, Clone, ::serde::Serialize, ::serde::Deserialize, PartialEq, Eq)]
                    #[serde(tag = "status", rename_all = "snake_case")]
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
                #[serde(tag = "cmd", rename_all = "snake_case")]
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
