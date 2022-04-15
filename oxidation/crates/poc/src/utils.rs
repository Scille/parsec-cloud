// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use proc_macro2::TokenStream;
use quote::quote;
use syn::{GenericArgument, Ident, PathArguments, Type};

use crate::parser::SchemaField;

// Extract the type recursively by changing all unit type except u8 by _
fn _extract_serde_as(ty: &Type) -> String {
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
                            GenericArgument::Type(ty) => _extract_serde_as(ty),
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
                .map(_extract_serde_as)
                .collect::<Vec<_>>()
                .join(",");
            ident.push(')');
            ident
        }
        ty => panic!("{ty:?} encountered"),
    }
}

// Extract a type for serde_as proc macro
fn extract_serde_as(ty: &Type) -> String {
    _extract_serde_as(ty)
        .replace("Vec<u8>", "::serde_with::Bytes")
        .replace("u8", "_")
}

pub(crate) fn quote_fields(fields: &SchemaField) -> TokenStream {
    let mut _fields = quote! {};

    for field in fields.fields.iter() {
        let name: Ident = syn::parse_str(&field.name).expect("Cannot convert into Ident");
        match field.fields.iter().find(|ty| ty.name == "type") {
            Some(ty) => {
                assert!(ty.fields.len() == 1, "`type` should contains 1 row");
                let ty: Type =
                    syn::parse_str(&ty.fields[0].name).expect("Cannot convert into Type");
                let serde_as = extract_serde_as(&ty);
                _fields = quote! {
                    #_fields
                    #[serde_as(as = #serde_as)]
                    pub #name: #ty,
                }
            }
            None => panic!("No `type` keyword found in your supposed struct field"),
        }
    }

    _fields
}

pub(crate) fn quote_struct(_struct: &SchemaField) -> TokenStream {
    let name: Ident = syn::parse_str(&_struct.name).expect("Cannot convert into Ident");
    let fields = match _struct.fields.iter().find(|field| field.name == "fields") {
        Some(fields) => quote_fields(fields),
        None => panic!("No `fields` keyword found in your supposed struct"),
    };

    quote! {
        #[::serde_with::serde_as]
        #[derive(Debug, Default, Clone, ::serde::Serialize, ::serde::Deserialize, PartialEq, Eq)]
        pub struct #name {
            #fields
        }
    }
}
