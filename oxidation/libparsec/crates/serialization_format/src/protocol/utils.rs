use std::collections::HashMap;

use proc_macro2::TokenStream;
use quote::quote;
use syn::{GenericArgument, PathArguments, Type};

pub fn validate_raw_type(
    raw_type: &str,
    types: &HashMap<String, String>,
) -> Result<String, String> {
    syn::parse_str(raw_type)
        .map_err(|e| format!("Invalid type value `{raw_type}`: {e}"))
        .and_then(|raw_type| inspect_type(&raw_type, types))
}

pub fn inspect_type(raw_type: &Type, types: &HashMap<String, String>) -> Result<String, String> {
    match raw_type {
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
                "PublicKey" => "libparsec_crypto::PublicKey",
                "SigningKey" => "libparsec_crypto::SigningKey",
                "VerifyKey" => "libparsec_crypto::VerifyKey",
                "PrivateKey" => "libparsec_crypto::PrivateKey",
                "SecretKey" => "libparsec_crypto::SecretKey",
                "HashDigest" => "libparsec_crypto::HashDigest",
                "DateTime" => "libparsec_types::DateTime",
                "BlockID" => "libparsec_types::BlockID",
                "DeviceID" => "libparsec_types::DeviceID",
                "EntryID" => "libparsec_types::EntryID",
                "UserID" => "libparsec_types::UserID",
                "RealmID" => "libparsec_types::RealmID",
                "VlobID" => "libparsec_types::VlobID",
                "SequesterServiceID" => "libparsec_types::SequesterServiceID",
                "DeviceLabel" => "libparsec_types::DeviceLabel",
                "HumanHandle" => "libparsec_types::HumanHandle",
                "UserProfile" => "libparsec_types::UserProfile",
                "RealmRole" => "libparsec_types::RealmRole",
                "InvitationToken" => "libparsec_types::InvitationToken",
                "InvitationStatus" => "libparsec_types::InvitationStatus",
                "ReencryptionBatchEntry" => "libparsec_types::ReencryptionBatchEntry",
                "CertificateSignerOwned" => "libparsec_types::CertificateSignerOwned",
                "BlockAccess" => "libparsec_types::BlockAccess",
                "EntryName" => "libparsec_types::EntryName",
                "WorkspaceEntry" => "libparsec_types::WorkspaceEntry",
                "FileManifest" => "libparsec_types::FileManifest",
                "FolderManifest" => "libparsec_types::FolderManifest",
                "WorkspaceManifest" => "libparsec_types::WorkspaceManifest",
                "UserManifest" => "libparsec_types::UserManifest",
                "Chunk" => "libparsec_client_types::Chunk",
                "BackendOrganizationAddr" => "libparsec_types::BackendOrganizationAddr",
                ident if types.get(ident).is_some() => {
                    types.get(ident).unwrap_or_else(|| unreachable!())
                }
                ident => return Err(format!("{ident} isn't allowed as type")),
            })
            .to_string();
            match &ty.arguments {
                PathArguments::None => Ok(ident),
                PathArguments::AngleBracketed(x) => {
                    ident.push('<');
                    ident += &x
                        .args
                        .iter()
                        .map(|arg| match arg {
                            GenericArgument::Type(ty) => inspect_type(ty, types),
                            _ => unimplemented!("for arg {:?}", arg),
                        })
                        .collect::<Result<Vec<_>, String>>()?
                        .join(",");
                    ident.push('>');
                    Ok(ident)
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
                .map(|ty| inspect_type(ty, types))
                .collect::<Result<Vec<_>, String>>()?
                .join(",");
            ident.push(')');
            Ok(ident)
        }
        ty => Err(format!("{ty:?} encountered")),
    }
}

pub fn quote_serde_as(ty: &Type) -> TokenStream {
    let serde_as = extract_serde_as(ty)
        .replace("Vec<u8>", "::serde_with::Bytes")
        .replace("u8", "_");
    quote!(#[serde_as(as = #serde_as)])
}

/// Extract the type recursively by changing all unit type except u8 by _
pub fn extract_serde_as(ty: &Type) -> String {
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
