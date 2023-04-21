// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use proc_macro2::TokenStream;
use std::collections::HashMap;
use syn::{GenericArgument, PathArguments, Type};

pub(crate) fn snake_to_pascal_case(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    // Start with capitalization
    let mut capitalize_next_char = true;
    for c in s.chars() {
        match (capitalize_next_char, c) {
            (_, '_') => {
                capitalize_next_char = true;
            }
            (true, c @ 'a'..='z') => {
                out.push((c as u8 - b'a' + b'A') as char);
                capitalize_next_char = false;
            }
            (_, c) => {
                out.push(c);
                capitalize_next_char = false;
            }
        }
    }
    // Weird corner case: `s` was only composed of underscores
    if out.is_empty() {
        out.push('_');
    }
    out
}

#[cfg(test)]
#[test]
fn test_snake_to_pascal_case() {
    assert_eq!(snake_to_pascal_case("a"), "A");
    assert_eq!(snake_to_pascal_case("#a!"), "#a!");
    assert_eq!(snake_to_pascal_case("a_!b"), "A!b");
    assert_eq!(snake_to_pascal_case("aa"), "Aa");
    assert_eq!(snake_to_pascal_case("a_b"), "AB");
    assert_eq!(snake_to_pascal_case("aa_bb_cCc"), "AaBbCCc");
    assert_eq!(snake_to_pascal_case("_a"), "A");
    assert_eq!(snake_to_pascal_case("_1_2b_3C"), "12b3C");
    assert_eq!(snake_to_pascal_case("_"), "_");
    assert_eq!(snake_to_pascal_case("___"), "_");
    assert_eq!(snake_to_pascal_case("__a__b__"), "AB");
    assert_eq!(snake_to_pascal_case("a_b_"), "AB");
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
                // Both value convert to the same type but have a different meaning.
                // - `RequiredOption` => The field must be present but its value can be null.
                // - `NonRequiredOption` => the field can be missing or its value to be null.
                "RequiredOption" | "NonRequiredOption" => "Option",
                "Option" => "Option",
                "List" => "Vec",
                "Map" => "::std::collections::HashMap",
                "Set" => "::std::collections::HashSet",
                "Version" => "u32",
                "Size" => "u64",
                "Index" => "u64",
                "NonZeroInteger" => "::std::num::NonZeroU64",
                "PublicKey" => "libparsec_types::PublicKey",
                "SigningKey" => "libparsec_types::SigningKey",
                "VerifyKey" => "libparsec_types::VerifyKey",
                "PrivateKey" => "libparsec_types::PrivateKey",
                "SecretKey" => "libparsec_types::SecretKey",
                "HashDigest" => "libparsec_types::HashDigest",
                "SequesterVerifyKeyDer" => "libparsec_types::SequesterVerifyKeyDer",
                "SequesterPublicKeyDer" => "libparsec_types::SequesterPublicKeyDer",
                "DateTime" => "libparsec_types::DateTime",
                "BlockID" => "libparsec_types::BlockID",
                "DeviceID" => "libparsec_types::DeviceID",
                "OrganizationID" => "libparsec_types::OrganizationID",
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
                "ActiveUsersLimit" => "libparsec_types::ActiveUsersLimit",
                "Chunk" => "libparsec_types::Chunk",
                "BackendOrganizationAddr" => "libparsec_types::BackendOrganizationAddr",
                "UsersPerProfileDetailItem" => "libparsec_types::UsersPerProfileDetailItem",
                "BackendPkiEnrollmentAddr" => "libparsec_types::BackendPkiEnrollmentAddr",
                "PkiEnrollmentSubmitPayload" => "PkiEnrollmentSubmitPayload",
                "X509Certificate" => "libparsec_types::X509Certificate",
                // Used only in protocol
                "IntegerBetween1And100" => "libparsec_types::IntegerBetween1And100",
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

pub(crate) fn quote_serde_as(ty: &Type, no_default: bool) -> TokenStream {
    let serde_as = extract_serde_as(ty)
        .replace("Vec<u8>", "::serde_with::Bytes")
        .replace("u8", "_");
    if no_default {
        syn::parse_quote!(#[serde_as(as = #serde_as, no_default)])
    } else {
        syn::parse_quote!(#[serde_as(as = #serde_as)])
    }
}
