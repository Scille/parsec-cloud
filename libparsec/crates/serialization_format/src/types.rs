// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};
use std::collections::HashSet;

macro_rules! generate_field_type_enum {
    ($($type_name: ident => $rust_type: path),+ $(,)?) => {
        #[derive(PartialEq, Eq, Clone)]
        pub(crate) enum FieldType {

            // Containers

            Map(Box<FieldType>, Box<FieldType>),
            List(Box<FieldType>),
            Set(Box<FieldType>),
            // Both value convert to the same type but have a different meaning.
            // - `RequiredOption` => The field must be present but its value can be null.
            // - `NonRequiredOption` => the field can be missing or its value to be null.
            RequiredOption(Box<FieldType>),
            NonRequiredOption(Box<FieldType>),
            Option(Box<FieldType>),
            Tuple(Vec<FieldType>),

            // Custom types

            Custom(String),

            // Scalars

            Boolean,
            String,
            Bytes,
            $($type_name),+
        }

        impl FieldType {
            pub fn from_json_type(s: &str, allowed_extra_types: Option<&HashSet<String>>) -> Self {
                match (s, allowed_extra_types) {
                    // Is a scalar ?
                    ("Boolean", _) => FieldType::Boolean,
                    ("String", _) => FieldType::String,
                    ("Bytes", _) => FieldType::Bytes,
                    $((stringify!($type_name), _) => FieldType::$type_name,)+

                    // Is a container ?
                    (s, _) if s.starts_with("Map<") && s.ends_with(">") => {
                        let (key, value) = s["Map<".len()..s.len() - 1].split_once(",").unwrap_or_else(|| panic!("Invalid type `{}`", s));
                        let key = FieldType::from_json_type(key.trim(), allowed_extra_types);
                        let value = FieldType::from_json_type(value.trim(), allowed_extra_types);
                        FieldType::Map(Box::new(key), Box::new(value))
                    },
                    (s, _) if s.starts_with("List<") && s.ends_with(">") => {
                        let nested = &s["List<".len()..s.len() - 1];
                        let nested = FieldType::from_json_type(nested.trim(), allowed_extra_types);
                        FieldType::List(Box::new(nested))
                    },
                    (s, _) if s.starts_with("Set<") && s.ends_with(">") => {
                        let nested = &s["Set<".len()..s.len() - 1];
                        let nested = FieldType::from_json_type(nested.trim(), allowed_extra_types);
                        FieldType::Set(Box::new(nested))
                    },
                    (s, _) if s.starts_with("RequiredOption<") && s.ends_with(">") => {
                        let nested = &s["RequiredOption<".len()..s.len() - 1];
                        let nested = FieldType::from_json_type(nested.trim(), allowed_extra_types);
                        FieldType::RequiredOption(Box::new(nested))
                    },
                    (s, _) if s.starts_with("NonRequiredOption<") && s.ends_with(">") => {
                        let nested = &s["NonRequiredOption<".len()..s.len() - 1];
                        let nested = FieldType::from_json_type(nested.trim(), allowed_extra_types);
                        FieldType::NonRequiredOption(Box::new(nested))
                    },
                    (s, _) if s.starts_with("Option<") && s.ends_with(">") => {
                        let nested = &s["Option<".len()..s.len() - 1];
                        let nested = FieldType::from_json_type(nested.trim(), allowed_extra_types);
                        FieldType::Option(Box::new(nested))
                    },
                    (s, _) if s.starts_with("(") && s.ends_with(")") => {
                        let elems = s[1..s.len()-1].split(",");
                        let elems = elems.map(|x| FieldType::from_json_type(x.trim(), allowed_extra_types)).collect();
                        FieldType::Tuple(elems)
                    }

                    // Custom type ?
                    (s, Some(allowed_extra_types)) if allowed_extra_types.get(s).is_some() => {
                        FieldType::Custom(s.to_owned())
                    }

                    // Invalid type :(
                    _ => panic!("Invalid type `{}`", s),
                }
            }

            pub fn to_rust_type(&self) -> TokenStream {
                match self {
                    // Scalars
                    FieldType::Boolean => quote!{ bool },
                    FieldType::String => quote!{ String },
                    FieldType::Bytes => quote!{ bytes::Bytes },
                    $(FieldType::$type_name => {
                        let (absolute_prefix, relative_path) = match stringify!($rust_type){
                            rust_type if rust_type.starts_with("::") => (true, &rust_type[2..]),
                            rust_type => (false, rust_type),
                        };

                        let relative_path_parts: Vec<Ident> = relative_path.split("::").map(|x| format_ident!("{}", x)).collect();
                        if absolute_prefix {
                            quote!{ ::#(#relative_path_parts)::*}
                        } else {
                            quote!{ #(#relative_path_parts)::*}
                        }
                    })+

                    // Containers

                    FieldType::Map(key, value) => {
                        let key = key.to_rust_type();
                        let value = value.to_rust_type();
                        quote! {
                            ::std::collections::HashMap<#key, #value>
                        }
                    }
                    FieldType::List(nested) => {
                        let nested = nested.to_rust_type();
                        quote!{
                            Vec<#nested>
                        }
                    }
                    FieldType::Set(nested) => {
                        let nested = nested.to_rust_type();
                        quote!{
                            ::std::collections::HashSet<#nested>
                        }
                    }
                    FieldType::RequiredOption(nested) => {
                        let nested = nested.to_rust_type();
                        quote!{
                            Option<#nested>
                        }
                    }
                    FieldType::NonRequiredOption(nested) => {
                        let nested = nested.to_rust_type();
                        quote!{
                            Option<#nested>
                        }
                    }
                    FieldType::Option(nested) => {
                        let nested = nested.to_rust_type();
                        quote!{
                            Option<#nested>
                        }
                    }
                    FieldType::Tuple(elems) => {
                        let elems = elems.iter().map(|e| e.to_rust_type()).collect::<Vec<TokenStream>>();
                        quote!{
                            (#(#elems),*)
                        }
                    }

                    // Custom type

                    FieldType::Custom(type_name) => {
                        let type_name = format_ident!("{}", type_name);
                        quote!{#type_name}
                    },
                }
            }

            // Provide pattern hint for serde_as conversion
            // e.g. `HashMap<String, Vec<u8>>` => `HashMap<_, ::serde_with::Bytes>`
            pub fn to_serde_as(&self) -> Option<TokenStream> {
                let to_serde_as_or_underscore = |nested: &FieldType| {
                    nested.to_serde_as().unwrap_or(quote!{ _ })
                };
                let code = match self {
                    FieldType::Map(key, value) => {
                        let key = to_serde_as_or_underscore(key);
                        let value = to_serde_as_or_underscore(value);
                        quote! {
                            ::std::collections::HashMap<#key, #value>
                        }
                    }
                    FieldType::List(nested) => {
                        let nested = to_serde_as_or_underscore(nested);
                        quote!{
                            Vec<#nested>
                        }
                    }
                    FieldType::Set(nested) => {
                        let nested = to_serde_as_or_underscore(nested);
                        quote!{
                            ::std::collections::HashSet<#nested>
                        }
                    }
                    FieldType::RequiredOption(nested) => {
                        let nested = to_serde_as_or_underscore(nested);
                        quote!{
                            Option<#nested>
                        }
                    }
                    FieldType::NonRequiredOption(nested) => {
                        let nested = to_serde_as_or_underscore(nested);
                        quote!{
                            Option<#nested>
                        }
                    }
                    FieldType::Option(nested) => {
                        let nested = to_serde_as_or_underscore(nested);
                        quote!{
                            Option<#nested>
                        }
                    }
                    FieldType::Tuple(elems) => {
                        let elems = elems.iter().map(to_serde_as_or_underscore).collect::<Vec<TokenStream>>();
                        quote!{
                            (#(#elems),*)
                        }
                    }
                    _ => return None,
                };
                Some(code)
            }
        }
    };
}

generate_field_type_enum!(
    // We don't provide an absolute path for the types (e.g. `::libparsec_types::Integer`)
    // This is because in our own test we mock `libparsec_types` to not depend on it.
    Integer => libparsec_types::Integer,
    Float => libparsec_types::Float,
    Version => libparsec_types::VersionInt,
    Size => libparsec_types::SizeInt,
    Index => libparsec_types::IndexInt,
    NonZeroInteger => ::std::num::NonZeroU64,
    PublicKey => libparsec_types::PublicKey,
    SigningKey => libparsec_types::SigningKey,
    VerifyKey => libparsec_types::VerifyKey,
    PrivateKey => libparsec_types::PrivateKey,
    SecretKey => libparsec_types::SecretKey,
    HashDigest => libparsec_types::HashDigest,
    SequesterVerifyKeyDer => libparsec_types::SequesterVerifyKeyDer,
    SequesterPublicKeyDer => libparsec_types::SequesterPublicKeyDer,
    DateTime => libparsec_types::DateTime,
    BlockID => libparsec_types::BlockID,
    DeviceID => libparsec_types::DeviceID,
    OrganizationID => libparsec_types::OrganizationID,
    UserID => libparsec_types::UserID,
    VlobID => libparsec_types::VlobID,
    EnrollmentID => libparsec_types::EnrollmentID,
    SequesterServiceID => libparsec_types::SequesterServiceID,
    DeviceLabel => libparsec_types::DeviceLabel,
    HumanHandle => libparsec_types::HumanHandle,
    UserProfile => libparsec_types::UserProfile,
    RealmRole => libparsec_types::RealmRole,
    InvitationToken => libparsec_types::InvitationToken,
    InvitationStatus => libparsec_types::InvitationStatus,
    ReencryptionBatchEntry => libparsec_types::ReencryptionBatchEntry,
    CertificateSignerOwned => libparsec_types::CertificateSignerOwned,
    BlockAccess => libparsec_types::BlockAccess,
    EntryName => libparsec_types::EntryName,
    WorkspaceEntry => libparsec_types::WorkspaceEntry,
    FileManifest => libparsec_types::FileManifest,
    FolderManifest => libparsec_types::FolderManifest,
    WorkspaceManifest => libparsec_types::WorkspaceManifest,
    UserManifest => libparsec_types::UserManifest,
    ActiveUsersLimit => libparsec_types::ActiveUsersLimit,
    Chunk => libparsec_types::Chunk,
    BackendOrganizationAddr => libparsec_types::BackendOrganizationAddr,
    UsersPerProfileDetailItem => libparsec_types::UsersPerProfileDetailItem,
    BackendPkiEnrollmentAddr => libparsec_types::BackendPkiEnrollmentAddr,
    PkiEnrollmentSubmitPayload => PkiEnrollmentSubmitPayload,
    X509Certificate => libparsec_types::X509Certificate,
    // Used only in protocol
    IntegerBetween1And100 => libparsec_types::IntegerBetween1And100,
);
