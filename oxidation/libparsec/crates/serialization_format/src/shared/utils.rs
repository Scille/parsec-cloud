// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

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

pub fn validate_raw_type(raw_type: &str, types: &HashMap<String, String>) -> anyhow::Result<Type> {
    syn::parse_str(raw_type)
        .map_err(|e| anyhow::anyhow!("Invalid raw type `{raw_type}`: {e}"))
        .and_then(|raw_type| inspect_type(&raw_type, types))
        .and_then(|raw_type| syn::parse_str(&raw_type).map_err(anyhow::Error::from))
}

pub fn inspect_type(raw_type: &Type, types: &HashMap<String, String>) -> anyhow::Result<String> {
    match raw_type {
        Type::Path(p) => {
            let ty = p.path.segments.last().unwrap_or_else(|| unreachable!());
            let mut ident = (match ty.ident.to_string().as_str() {
                "Integer" => "i64",
                "Boolean" => "bool",
                "Float" => "f64",
                "String" => "String",
                "Bytes" => "Vec<u8>",
                // Both value convert to the same type but have a different meaning.
                // - `RequiredOption` => The field must be present but its value can be null.
                // - `NonRequiredOption` => the field can be missing or its value to be null.
                "RequiredOption" | "NonRequiredOption" => "Option",
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
                "EnrollmentID" => "libparsec_types::EnrollmentID",
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
                // Used only in protocol
                "IntegerBetween1And100" => "crate::IntegerBetween1And100",
                ident if types.get(ident).is_some() => {
                    types.get(ident).unwrap_or_else(|| unreachable!())
                }
                ident => return Err(anyhow::anyhow!("{ident} isn't allowed as type")),
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
                .map(|ty| inspect_type(ty, types))
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
