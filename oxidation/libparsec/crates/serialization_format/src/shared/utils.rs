// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use std::collections::HashMap;

use syn::{GenericArgument, PathArguments, Type};

use crate::config::CratesPaths;

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

pub fn validate_raw_type(
    raw_type: &str,
    types: &HashMap<String, String>,
    crates_paths: &CratesPaths,
) -> anyhow::Result<Type> {
    syn::parse_str(raw_type)
        .map_err(|e| anyhow::anyhow!("Invalid raw type `{raw_type}`: {e}"))
        .and_then(|raw_type| inspect_type(&raw_type, types, crates_paths))
        .and_then(|raw_type| syn::parse_str(&raw_type).map_err(anyhow::Error::from))
}

pub fn inspect_type(
    raw_type: &Type,
    types: &HashMap<String, String>,
    crates_paths: &CratesPaths,
) -> anyhow::Result<String> {
    let libparsec_types = &crates_paths["libparsec_types"];
    let libparsec_crypto = &crates_paths["libparsec_crypto"];
    let libparsec_client_types = &crates_paths["libparsec_client_types"];

    match raw_type {
        Type::Path(p) => {
            let ty = p.path.segments.last().unwrap_or_else(|| unreachable!());
            let mut ident = match ty.ident.to_string().as_str() {
                "Integer" => "i64".to_string(),
                "Boolean" => "bool".to_string(),
                "Float" => "f64".to_string(),
                "String" => "String".to_string(),
                "Bytes" => "Vec<u8>".to_string(),
                // Both value convert to the same type but have a different meaning.
                // - `RequiredOption` => The field must be present but its value can be null.
                // - `NonRequiredOption` => the field can be missing or its value to be null.
                "RequiredOption" | "NonRequiredOption" => "Option".to_string(),
                "List" => "Vec".to_string(),
                "Map" => "::std::collections::HashMap".to_string(),
                "Set" => "::std::collections::HashSet".to_string(),
                "Version" => "u32".to_string(),
                "Size" => "u64".to_string(),
                "Index" => "u64".to_string(),
                "NonZeroInteger" => "::std::num::NonZeroU64".to_string(),
                "PublicKey" => format!("{libparsec_crypto}::PublicKey"),
                "SigningKey" => format!("{libparsec_crypto}::SigningKey"),
                "VerifyKey" => format!("{libparsec_crypto}::VerifyKey"),
                "PrivateKey" => format!("{libparsec_crypto}::PrivateKey"),
                "SecretKey" => format!("{libparsec_crypto}::SecretKey"),
                "HashDigest" => format!("{libparsec_crypto}::HashDigest"),
                "DateTime" => format!("{libparsec_types}::DateTime"),
                "BlockID" => format!("{libparsec_types}::BlockID"),
                "DeviceID" => format!("{libparsec_types}::DeviceID"),
                "EntryID" => format!("{libparsec_types}::EntryID"),
                "UserID" => format!("{libparsec_types}::UserID"),
                "RealmID" => format!("{libparsec_types}::RealmID"),
                "VlobID" => format!("{libparsec_types}::VlobID"),
                "EnrollmentID" => format!("{libparsec_types}::EnrollmentID"),
                "SequesterServiceID" => format!("{libparsec_types}::SequesterServiceID"),
                "DeviceLabel" => format!("{libparsec_types}::DeviceLabel"),
                "HumanHandle" => format!("{libparsec_types}::HumanHandle"),
                "UserProfile" => format!("{libparsec_types}::UserProfile"),
                "RealmRole" => format!("{libparsec_types}::RealmRole"),
                "InvitationToken" => format!("{libparsec_types}::InvitationToken"),
                "InvitationStatus" => format!("{libparsec_types}::InvitationStatus"),
                "ReencryptionBatchEntry" => format!("{libparsec_types}::ReencryptionBatchEntry"),
                "CertificateSignerOwned" => format!("{libparsec_types}::CertificateSignerOwned"),
                "BlockAccess" => format!("{libparsec_types}::BlockAccess"),
                "EntryName" => format!("{libparsec_types}::EntryName"),
                "WorkspaceEntry" => format!("{libparsec_types}::WorkspaceEntry"),
                "FileManifest" => format!("{libparsec_types}::FileManifest"),
                "FolderManifest" => format!("{libparsec_types}::FolderManifest"),
                "WorkspaceManifest" => format!("{libparsec_types}::WorkspaceManifest"),
                "UserManifest" => format!("{libparsec_types}::UserManifest"),
                "Chunk" => format!("{libparsec_client_types}::Chunk"),
                "BackendOrganizationAddr" => format!("{libparsec_types}::BackendOrganizationAddr"),
                // Used only in protocol
                "IntegerBetween1And100" => "crate::IntegerBetween1And100".to_string(),
                ident if types.get(ident).is_some() => types
                    .get(ident)
                    .unwrap_or_else(|| unreachable!())
                    .to_string(),
                ident => return Err(anyhow::anyhow!("{ident} isn't allowed as type")),
            };
            match &ty.arguments {
                PathArguments::None => Ok(ident),
                PathArguments::AngleBracketed(x) => {
                    ident.push('<');
                    ident += &x
                        .args
                        .iter()
                        .map(|arg| match arg {
                            GenericArgument::Type(ty) => inspect_type(ty, types, crates_paths),
                            _ => unimplemented!("for arg {:?}", arg),
                        })
                        .collect::<anyhow::Result<Vec<_>>>()?
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
                .map(|ty| inspect_type(ty, types, crates_paths))
                .collect::<anyhow::Result<Vec<_>>>()?
                .join(",");
            ident.push(')');
            Ok(ident)
        }
        ty => Err(anyhow::anyhow!("{ty:?} encountered")),
    }
}

pub fn quote_serde_as(ty: &Type, no_default: bool) -> anyhow::Result<syn::Attribute> {
    let serde_as = extract_serde_as(ty)?
        .replace("Vec<u8>", "::serde_with::Bytes")
        .replace("u8", "_");
    if no_default {
        Ok(syn::parse_quote!(#[serde_as(as = #serde_as, no_default)]))
    } else {
        Ok(syn::parse_quote!(#[serde_as(as = #serde_as)]))
    }
}

/// Extract the type recursively by changing all unit type except u8 by _
pub fn extract_serde_as(ty: &Type) -> anyhow::Result<String> {
    let res = match ty {
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
                        .collect::<anyhow::Result<Vec<_>>>()?
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
                .collect::<anyhow::Result<Vec<_>>>()?
                .join(",");
            ident.push(')');
            ident
        }
        ty => return Err(anyhow::anyhow!("{ty:?} encountered")),
    };

    Ok(res)
}

#[cfg(test)]
#[rstest::rstest]
#[case::byte("u8", "u8")]
#[case::string("String", "_")]
#[case::custom("InvitationEmailSentStatus", "_")]
fn test_extract_serde_as(#[case] raw_type: &str, #[case] expected: &str) {
    use pretty_assertions::assert_eq;

    let ty = syn::parse_str::<Type>(raw_type).expect("A valid type to be parsed by syn");
    let res = extract_serde_as(&ty).unwrap();

    assert_eq!(res.as_str(), expected)
}
