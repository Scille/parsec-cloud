// Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use serde::{Deserialize, Serialize};
use serde_with::*;
use std::io::{Read, Write};

use parsec_api_crypto::{PublicKey, SigningKey, VerifyKey};

use crate::data_macros::{impl_transparent_data_format_conversion, new_data_struct_type};
use crate::ext_types::maybe_field;
use crate::{
    DateTime, DeviceID, DeviceLabel, EntryID, HumanHandle, RealmRole, UserID, UserProfile,
};

#[allow(unused_macros)]
macro_rules! impl_verify_and_load_allow_root {
    ($name:ident) => {
        impl $name {
            pub fn verify_and_load<'a>(
                signed: &[u8],
                author_verify_key: &VerifyKey,
                expected_author: CertificateSignerRef<'a>,
            ) -> Result<$name, &'static str> {
                let compressed = author_verify_key
                    .verify(signed)
                    .map_err(|_| "Invalid signature")?;
                let mut serialized = vec![];
                ZlibDecoder::new(&compressed[..])
                    .read_to_end(&mut serialized)
                    .map_err(|_| "Invalid compression")?;
                let obj: $name =
                    ::rmp_serde::from_read_ref(&serialized).map_err(|_| "Invalid serialization")?;
                match (&obj.author, expected_author) {
                    (CertificateSignerOwned::User(ref a_id), CertificateSignerRef::User(ea_id))
                        if a_id == ea_id =>
                    {
                        Ok(obj)
                    }
                    (CertificateSignerOwned::Root, CertificateSignerRef::Root) => Ok(obj),
                    _ => Err("Unexpected author"),
                }
            }
        }
    };
}

macro_rules! impl_verify_and_load_no_root {
    ($name:ident) => {
        impl $name {
            pub fn verify_and_load(
                signed: &[u8],
                author_verify_key: &VerifyKey,
                expected_author: &DeviceID,
            ) -> Result<$name, &'static str> {
                let compressed = author_verify_key
                    .verify(signed)
                    .map_err(|_| "Invalid signature")?;
                let mut serialized = vec![];
                ZlibDecoder::new(&compressed[..])
                    .read_to_end(&mut serialized)
                    .map_err(|_| "Invalid compression")?;
                let obj: $name =
                    ::rmp_serde::from_read_ref(&serialized).map_err(|_| "Invalid serialization")?;
                if &obj.author != expected_author {
                    Err("Unexpected author")
                } else {
                    Ok(obj)
                }
            }
        }
    };
}

macro_rules! impl_unsecure_load {
    ($name:ident) => {
        impl $name {
            pub fn unsecure_load(signed: &[u8]) -> Result<$name, &'static str> {
                let compressed = VerifyKey::unsecure_unwrap(signed).ok_or("Invalid signature")?;
                let mut serialized = vec![];
                ZlibDecoder::new(&compressed[..])
                    .read_to_end(&mut serialized)
                    .map_err(|_| "Invalid compression")?;
                ::rmp_serde::from_read_ref(&serialized).map_err(|_| "Invalid serialization")
            }
        }
    };
}

macro_rules! impl_dump_and_sign {
    ($name:ident) => {
        impl $name {
            pub fn dump_and_sign(&self, author_signkey: &SigningKey) -> Vec<u8> {
                let serialized =
                    ::rmp_serde::to_vec_named(&self).unwrap_or_else(|_| unreachable!());
                let mut e = ZlibEncoder::new(Vec::new(), flate2::Compression::default());
                e.write_all(&serialized).unwrap_or_else(|_| unreachable!());
                let compressed = e.finish().unwrap_or_else(|_| unreachable!());
                author_signkey.sign(&compressed)
            }
        }
    };
}

/*
 * CertificateSigner
 */

// Signature can be done either by a user (through one of it devices) or
// by the Root Key when bootstrapping the organization (only the very first
// user and device certificates are signed this way)

pub enum CertificateSignerRef<'a> {
    User(&'a DeviceID),
    Root,
}

impl<'a> From<Option<&'a DeviceID>> for CertificateSignerRef<'a> {
    fn from(item: Option<&'a DeviceID>) -> CertificateSignerRef {
        match item {
            Some(device_id) => CertificateSignerRef::User(device_id),
            None => CertificateSignerRef::Root,
        }
    }
}

impl<'a> From<CertificateSignerRef<'a>> for Option<&'a DeviceID> {
    fn from(item: CertificateSignerRef<'a>) -> Option<&'a DeviceID> {
        match item {
            CertificateSignerRef::User(device_id) => Some(device_id),
            CertificateSignerRef::Root => None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(into = "Option<DeviceID>", try_from = "Option<DeviceID>")]
pub enum CertificateSignerOwned {
    User(DeviceID),
    Root,
}

impl From<Option<DeviceID>> for CertificateSignerOwned {
    fn from(item: Option<DeviceID>) -> CertificateSignerOwned {
        match item {
            Some(device_id) => CertificateSignerOwned::User(device_id),
            None => CertificateSignerOwned::Root,
        }
    }
}

impl From<CertificateSignerOwned> for Option<DeviceID> {
    fn from(item: CertificateSignerOwned) -> Option<DeviceID> {
        match item {
            CertificateSignerOwned::User(device_id) => Some(device_id),
            CertificateSignerOwned::Root => None,
        }
    }
}

/*
 * UserCertificate
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(into = "UserCertificateData", from = "UserCertificateData")]
pub struct UserCertificate {
    pub author: CertificateSignerOwned,
    pub timestamp: DateTime,

    pub user_id: UserID,
    // Human handle can be none in case of redacted certificate
    pub human_handle: Option<HumanHandle>,
    pub public_key: PublicKey,
    pub profile: UserProfile,
}

impl_verify_and_load_allow_root!(UserCertificate);
impl_unsecure_load!(UserCertificate);
impl_dump_and_sign!(UserCertificate);

new_data_struct_type!(
    UserCertificateData,
    type: "user_certificate",

    author: CertificateSignerOwned,
    timestamp: DateTime,

    user_id: UserID,
    // Added in Parsec v1.13
    #[serde(default, deserialize_with = "maybe_field::deserialize_some")]
    human_handle: Option<Option<HumanHandle>>,
    public_key: PublicKey,
    // `profile` replaces `is_admin` field (which is still required for
    // backward compatibility)
    is_admin: bool,
    // Added in Parsec v1.14
    #[serde(default, deserialize_with = "maybe_field::deserialize_some")]
    profile: Option<UserProfile>,
);

impl From<UserCertificateData> for UserCertificate {
    fn from(data: UserCertificateData) -> Self {
        let profile = data.profile.unwrap_or(match data.is_admin {
            true => UserProfile::Admin,
            false => UserProfile::Standard,
        });
        Self {
            author: data.author,
            timestamp: data.timestamp,
            user_id: data.user_id,
            human_handle: data.human_handle.unwrap_or_default(),
            public_key: data.public_key,
            profile,
        }
    }
}

impl From<UserCertificate> for UserCertificateData {
    fn from(obj: UserCertificate) -> Self {
        Self {
            type_: Default::default(),
            author: obj.author,
            timestamp: obj.timestamp,
            user_id: obj.user_id,
            human_handle: Some(obj.human_handle),
            public_key: obj.public_key,
            profile: Some(obj.profile),
            is_admin: obj.profile == UserProfile::Admin,
        }
    }
}

/*
 * RevokedUserCertificate
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(
    into = "RevokedUserCertificateData",
    from = "RevokedUserCertificateData"
)]
pub struct RevokedUserCertificate {
    pub author: DeviceID,
    pub timestamp: DateTime,

    pub user_id: UserID,
}

impl_verify_and_load_no_root!(RevokedUserCertificate);
impl_unsecure_load!(RevokedUserCertificate);
impl_dump_and_sign!(RevokedUserCertificate);

new_data_struct_type!(
    RevokedUserCertificateData,
    type: "revoked_user_certificate",

    author: DeviceID,
    timestamp: DateTime,

    user_id: UserID,
);

impl_transparent_data_format_conversion!(
    RevokedUserCertificate,
    RevokedUserCertificateData,
    author,
    timestamp,
    user_id,
);

/*
 * DeviceCertificate
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(into = "DeviceCertificateData", from = "DeviceCertificateData")]
pub struct DeviceCertificate {
    pub author: CertificateSignerOwned,
    pub timestamp: DateTime,

    pub device_id: DeviceID,
    // Device label can be none in case of redacted certificate
    pub device_label: Option<DeviceLabel>,
    pub verify_key: VerifyKey,
}

impl_verify_and_load_allow_root!(DeviceCertificate);
impl_unsecure_load!(DeviceCertificate);
impl_dump_and_sign!(DeviceCertificate);

new_data_struct_type!(
    DeviceCertificateData,
    type: "device_certificate",

    author: CertificateSignerOwned,
    timestamp: DateTime,

    device_id: DeviceID,
    // Added in Parsec v1.14
    #[serde(default, deserialize_with = "maybe_field::deserialize_some")]
    device_label: Option<Option<DeviceLabel>>,
    verify_key: VerifyKey,
);

impl From<DeviceCertificateData> for DeviceCertificate {
    fn from(data: DeviceCertificateData) -> Self {
        Self {
            author: data.author,
            timestamp: data.timestamp,
            device_id: data.device_id,
            device_label: data.device_label.unwrap_or_default(),
            verify_key: data.verify_key,
        }
    }
}

impl From<DeviceCertificate> for DeviceCertificateData {
    fn from(obj: DeviceCertificate) -> Self {
        Self {
            type_: Default::default(),
            author: obj.author,
            timestamp: obj.timestamp,
            device_id: obj.device_id,
            device_label: Some(obj.device_label),
            verify_key: obj.verify_key,
        }
    }
}

/*
 * RealmRoleCertificate
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(into = "RealmRoleCertificateData", from = "RealmRoleCertificateData")]
pub struct RealmRoleCertificate {
    pub author: DeviceID,
    pub timestamp: DateTime,

    pub realm_id: EntryID,
    pub user_id: UserID,
    // Set to None if role removed
    pub role: Option<RealmRole>, // TODO: use a custom type instead
}

impl_verify_and_load_no_root!(RealmRoleCertificate);
impl_unsecure_load!(RealmRoleCertificate);
impl_dump_and_sign!(RealmRoleCertificate);

new_data_struct_type!(
    RealmRoleCertificateData,
    type: "realm_role_certificate",

    author: DeviceID,
    timestamp: DateTime,

    realm_id: EntryID,
    user_id: UserID,
    // Set to None if role removed
    role: Option<RealmRole>,  // TODO: use a custom type instead
);

impl_transparent_data_format_conversion!(
    RealmRoleCertificate,
    RealmRoleCertificateData,
    author,
    timestamp,
    realm_id,
    user_id,
    role,
);
