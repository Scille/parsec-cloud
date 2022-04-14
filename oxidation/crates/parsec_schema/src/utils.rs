// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use proc_macro2::TokenStream;
use quote::quote;
use std::collections::HashMap;
use syn::punctuated::{IntoIter, Iter};
use syn::{Attribute, Field, Fields, GenericArgument, Ident, PathArguments, Type, Variant};

use crate::parser;

fn extract_ty(ty: &Type) -> String {
    match ty {
        Type::Path(p) => {
            let ty = p.path.segments.last().unwrap_or_else(|| unreachable!());
            let mut ident = ty.ident.to_string();
            match &ty.arguments {
                PathArguments::None => ident,
                PathArguments::AngleBracketed(x) => {
                    ident.push('<');
                    ident += &x
                        .args
                        .iter()
                        .map(|arg| match arg {
                            GenericArgument::Type(ty) => extract_ty(ty),
                            _ => unimplemented!(),
                        })
                        .collect::<Vec<_>>()
                        .join(",");
                    ident.push('>');
                    ident
                }
                PathArguments::Parenthesized(x) => {
                    ident.push('(');
                    ident += &x
                        .inputs
                        .iter()
                        .map(extract_ty)
                        .collect::<Vec<_>>()
                        .join(",");
                    ident.push(')');
                    ident
                }
            }
        }
        Type::Tuple(t) => {
            let mut ident = String::new();
            ident.push('(');
            ident += &t.elems.iter().map(extract_ty).collect::<Vec<_>>().join(",");
            ident.push(')');
            ident
        }
        ty => panic!("{ty:?} encountered"),
    }
}

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
                PathArguments::Parenthesized(x) => {
                    ident.push('(');
                    ident += &x
                        .inputs
                        .iter()
                        .map(extract_serde_as)
                        .collect::<Vec<_>>()
                        .join(",");
                    ident.push(')');
                    ident
                }
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

pub(crate) fn extract_extra_specs(ident: &Ident) -> TokenStream {
    let ident = ident.to_string();
    let ident_len = ident.len() - 3;
    if &ident[ident_len..] == "Req" {
        let mut cmd = ident[..1].to_lowercase();
        ident.chars().take(ident_len).skip(1).for_each(|c| match c {
            c @ 'A'..='Z' => {
                cmd.push('_');
                cmd.push((c as u8 - b'A' + b'a') as char)
            }
            c @ '0'..='9' => {
                cmd.push('_');
                cmd.push(c)
            }
            c => cmd.push(c),
        });
        quote! {
            "cmd": {
                "type": "CheckedConstant",
                "value": #cmd,
            },
        }
    } else if &ident[ident_len..] == "Rep" {
        quote! {
            "status": {
                "type": "CheckedConstant",
                "value": "ok",
            },
        }
    } else {
        quote! {}
    }
}

pub(crate) fn quote_fields(fields: Iter<Field>) -> (TokenStream, TokenStream) {
    let mut _fields = quote! {};
    let mut _specs = quote! {};

    for (i, field) in fields.enumerate() {
        let ty = extract_ty(&field.ty);
        let _ident = field
            .ident
            .as_ref()
            .map(|ident| ident.to_string())
            .unwrap_or(format!("{i}"));

        let serde_as = extract_serde_as(&field.ty).replace("Vec<u8>", "::serde_with::Bytes");
        _fields = quote! {
            #_fields
            #[serde_as(as = #serde_as)]
            #field,
        };

        _specs = quote! {
            #_specs
            #_ident: {
                "type": #ty,
            },
        }
    }

    (_fields, _specs)
}

pub(crate) fn quote_variants(variants: IntoIter<Variant>) -> (TokenStream, TokenStream) {
    let mut _variants = quote! {};
    let mut _specs = quote! {};

    for variant in variants {
        let ident = &variant.ident;
        let (_attrs, _) = quote_attrs(variant.attrs);
        let (_fields, _current_specs) = quote_fields(variant.fields.iter());

        if ident.to_string().as_str() == "Ok" {
            _specs = _current_specs
        }

        match variant.fields {
            Fields::Unit => {
                _variants = quote! {
                    #_variants
                    #_attrs
                    #ident,
                }
            }
            Fields::Unnamed(_) => {
                _variants = quote! {
                    #_variants
                    #_attrs
                    #ident (#_fields),
                }
            }
            Fields::Named(_) => {
                _variants = quote! {
                    #_variants
                    #_attrs
                    #ident {#_fields},
                }
            }
        }
    }

    (_variants, _specs)
}

pub(crate) fn quote_attrs(attrs: Vec<Attribute>) -> (TokenStream, HashMap<String, String>) {
    let mut _attrs = quote! {};
    let mut serde_attrs = HashMap::new();

    for attr in attrs {
        _attrs = quote! {
            #_attrs
            #attr
        };

        // MacroAttribute should have a name
        let name = attr.path.segments.last().unwrap_or_else(|| unreachable!());
        if name.ident.to_string().as_str() == "serde" {
            let attr: parser::SerdeAttr =
                syn::parse2(attr.tokens).unwrap_or_else(|_| unreachable!());
            for field in attr.fields {
                serde_attrs.insert(field.ident.to_string(), field.literal.value());
            }
        }
    }

    (_attrs, serde_attrs)
}
