// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use miniserde::Deserialize;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{GenericArgument, Ident, PathArguments, Type};

fn to_pascal_case(s: &str) -> String {
    let mut out = s[..1].to_uppercase();
    let mut chars = s.chars().skip(1);
    while let Some(c) = chars.next() {
        if c == '_' {
            match chars.next().unwrap_or_else(|| unreachable!()) {
                c @ 'a'..='z' => out.push((c as u8 - b'a' + b'A') as char),
                c => out.push(c),
            }
        } else {
            out.push(c);
        }
    }

    out
}

fn inspect_type(ty: &Type) -> String {
    match ty {
        Type::Path(p) => {
            let ty = p.path.segments.last().unwrap_or_else(|| unreachable!());
            let mut ident = (match ty.ident.to_string().as_str() {
                "Integer" => "i64",
                "Boolean" => "bool",
                "Float" => "f64",
                "String" => "String",
                "Bytes" => "Vec<u8>",
                "Option" => "Option",
                "List" => "Vec",
                "Map" => "::std::collections::HashMap",
                "Set" => "::std::collections::HashSet",
                "BlockID" => "::parsec_api_types::BlockID",
                ident => panic!("{ident} isn't allowed as type"),
            })
            .to_string();
            match &ty.arguments {
                PathArguments::None => ident,
                PathArguments::AngleBracketed(x) => {
                    ident.push('<');
                    ident += &x
                        .args
                        .iter()
                        .map(|arg| match arg {
                            GenericArgument::Type(ty) => inspect_type(ty),
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
                .map(inspect_type)
                .collect::<Vec<_>>()
                .join(",");
            ident.push(')');
            ident
        }
        ty => panic!("{ty:?} encountered"),
    }
}

// Extract the type recursively by changing all unit type except u8 by _
fn extract_serde_as(ty: &Type) -> String {
    match ty {
        Type::Path(p) => {
            let ty = p.path.segments.last().unwrap_or_else(|| unreachable!());
            let mut ident = p
                .path
                .segments
                .iter()
                .map(|seg| seg.ident.to_string())
                .collect::<Vec<_>>()
                .join("::");
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
    Tag,
}

fn quote_serde_attr(attr: SerdeAttr, value: Option<&str>) -> TokenStream {
    match value {
        Some(value) => match attr {
            SerdeAttr::Rename => quote! { #[serde(rename = #value)] },
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
    introduced_in_revision: Option<u32>,
}

impl Field {
    fn quote(&self, vis: Vis) -> TokenStream {
        let rename = match self.name.as_str() {
            "type" => Some("type_"),
            _ => None,
        };
        let name: Ident =
            syn::parse_str(rename.unwrap_or(&self.name)).unwrap_or_else(|_| unreachable!());
        let raw_ty: Type = syn::parse_str(&self.ty).unwrap_or_else(|e| panic!("{e}"));
        let (inspected_ty, serde_skip) = if self.introduced_in_revision.is_some() {
            (
                "::parsec_api_types::Maybe".to_string() + "<" + &inspect_type(&raw_ty) + ">",
                quote! {
                    #[serde(default, skip_serializing_if = "::parsec_api_types::Maybe::is_absent")]
                },
            )
        } else {
            (inspect_type(&raw_ty), quote! {})
        };
        let ty: Type = syn::parse_str(&inspected_ty).unwrap_or_else(|e| panic!("{e}"));
        let rename = quote_serde_attr(SerdeAttr::Rename, rename.map(|_| &self.name[..]));
        let serde_as = quote_serde_as(&ty);

        match vis {
            Vis::Public => quote! {
                #rename
                #serde_as
                #serde_skip
                pub #name: #ty
            },
            Vis::Private => quote! {
                #rename
                #serde_as
                #serde_skip
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
    fields: Vec<Field>,
}

impl Variant {
    fn quote(&self) -> TokenStream {
        let rename = match self.name.as_str() {
            "type" => Some("type_"),
            _ => None,
        };
        let name: Ident =
            syn::parse_str(rename.unwrap_or(&self.name)).unwrap_or_else(|_| unreachable!());
        let rename = quote_serde_attr(SerdeAttr::Rename, rename.map(|_| &self.name[..]));
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
    // Struct based
    fields: Option<Vec<Field>>,
    // Enum based
    tag: Option<String>,
    variants: Option<Vec<Variant>>,
}

impl Schema {
    pub(crate) fn quote(&self) -> TokenStream {
        let name: Ident = syn::parse_str(&self.name).unwrap_or_else(|_| unreachable!());
        let tag = quote_serde_attr(SerdeAttr::Tag, self.tag.as_ref().map(|tag| &tag[..]));

        if let Some(variants) = &self.variants {
            let variants = quote_variants(variants);

            quote! {
                #[::serde_with::serde_as]
                #[derive(Debug, Clone, ::serde::Serialize, ::serde::Deserialize, PartialEq, Eq)]
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
        let rename = quote_serde_attr(SerdeAttr::Rename, Some(&self.status));
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
