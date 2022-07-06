// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use proc_macro2::TokenStream;
use quote::quote;
use std::collections::HashMap;
use syn::{GenericArgument, PathArguments, Type};

pub(crate) fn to_pascal_case(s: &str) -> String {
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

pub(crate) fn inspect_type(ty: &str, types: &HashMap<String, String>) -> String {
    let raw_ty: Type =
        syn::parse_str(ty).unwrap_or_else(|e| panic!("Invalid type value `{ty}`: {e}"));
    _inspect_type(&raw_ty, types)
}

pub(crate) fn _inspect_type(ty: &Type, types: &HashMap<String, String>) -> String {
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
                "Version" => "u32",
                "Size" => "u64",
                "Index" => "u64",
                "NonZeroInteger" => "::std::num::NonZeroU64",
                "PublicKey" => "api_crypto::PublicKey",
                "VerifyKey" => "api_crypto::VerifyKey",
                "PrivateKey" => "api_crypto::PrivateKey",
                "SecretKey" => "api_crypto::SecretKey",
                "HashDigest" => "api_crypto::HashDigest",
                "DateTime" => "api_types::DateTime",
                "BlockID" => "api_types::BlockID",
                "DeviceID" => "api_types::DeviceID",
                "EntryID" => "api_types::EntryID",
                "UserID" => "api_types::UserID",
                "RealmID" => "api_types::RealmID",
                "VlobID" => "api_types::VlobID",
                "DeviceLabel" => "api_types::DeviceLabel",
                "HumanHandle" => "api_types::HumanHandle",
                "UserProfile" => "api_types::UserProfile",
                "RealmRole" => "api_types::RealmRole",
                "InvitationToken" => "api_types::InvitationToken",
                "InvitationStatus" => "api_types::InvitationStatus",
                "ReencryptionBatchEntry" => "api_types::ReencryptionBatchEntry",
                "CertificateSignerOwned" => "api_types::CertificateSignerOwned",
                "BlockAccess" => "api_types::BlockAccess",
                "EntryName" => "api_types::EntryName",
                "WorkspaceEntry" => "api_types::WorkspaceEntry",
                "FileManifest" => "api_types::FileManifest",
                "FolderManifest" => "api_types::FolderManifest",
                "WorkspaceManifest" => "api_types::WorkspaceManifest",
                "UserManifest" => "api_types::UserManifest",
                "Chunk" => "parsec_client_types::Chunk",
                ident if types.get(ident).is_some() => {
                    types.get(ident).unwrap_or_else(|| unreachable!())
                }
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
                            GenericArgument::Type(ty) => _inspect_type(ty, types),
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
                .map(|ty| _inspect_type(ty, types))
                .collect::<Vec<_>>()
                .join(",");
            ident.push(')');
            ident
        }
        ty => panic!("{ty:?} encountered"),
    }
}

// Extract the type recursively by changing all unit type except u8 by _
pub(crate) fn extract_serde_as(ty: &Type) -> String {
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

pub(crate) fn quote_serde_as(ty: &Type) -> TokenStream {
    let serde_as = extract_serde_as(ty)
        .replace("Vec<u8>", "::serde_with::Bytes")
        .replace("u8", "_");
    quote! { #[serde_as(as = #serde_as)] }
}
