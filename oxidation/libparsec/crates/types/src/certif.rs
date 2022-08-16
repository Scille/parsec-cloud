// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use serde::{Deserialize, Serialize};
use serde_with::*;
use std::io::{Read, Write};

use libparsec_crypto::{PublicKey, SigningKey, VerifyKey};
use serialization_format::parsec_data;

use crate as libparsec_types;
use crate::data_macros::impl_transparent_data_format_conversion;
use crate::{DataError, DataResult};
use crate::{
    DateTime, DeviceID, DeviceLabel, HumanHandle, RealmID, RealmRole, UserID, UserProfile,
};

#[allow(unused_macros)]
macro_rules! impl_verify_and_load_allow_root {
    ($name:ident) => {
        impl $name {
            pub fn verify_and_load<'a>(
                signed: &[u8],
                author_verify_key: &VerifyKey,
                expected_author: CertificateSignerRef<'a>,
            ) -> DataResult<$name> {
                let compressed = author_verify_key.verify(signed)?;
                let mut serialized = vec![];
                ZlibDecoder::new(&compressed[..])
                    .read_to_end(&mut serialized)
                    .map_err(|_| DataError::Compression)?;
                let obj: $name =
                    ::rmp_serde::from_slice(&serialized).map_err(|_| DataError::Serialization)?;
                match (&obj.author, expected_author) {
                    (CertificateSignerOwned::User(ref a_id), CertificateSignerRef::User(ea_id)) => {
                        if a_id == ea_id {
                            Ok(obj)
                        } else {
                            Err(DataError::UnexpectedAuthor {
                                expected: ea_id.clone(),
                                got: Some(a_id.clone()),
                            })
                        }
                    }
                    (_, CertificateSignerRef::Root) => Ok(obj),
                    (_, CertificateSignerRef::User(ea_id)) => Err(DataError::UnexpectedAuthor {
                        expected: ea_id.clone(),
                        got: None,
                    }),
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
            ) -> DataResult<$name> {
                let compressed = author_verify_key.verify(signed)?;
                let mut serialized = vec![];
                ZlibDecoder::new(&compressed[..])
                    .read_to_end(&mut serialized)
                    .map_err(|_| DataError::Compression)?;
                let obj: $name =
                    ::rmp_serde::from_slice(&serialized).map_err(|_| DataError::Serialization)?;
                if &obj.author != expected_author {
                    Err(DataError::UnexpectedAuthor {
                        expected: expected_author.clone(),
                        got: Some(obj.author.clone()),
                    })
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
                ::rmp_serde::from_slice(&serialized).map_err(|_| "Invalid serialization")
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

parsec_data!("schema/certif/user_certificate.json");

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
            human_handle: data.human_handle.into(),
            public_key: data.public_key,
            profile,
        }
    }
}

impl From<UserCertificate> for UserCertificateData {
    fn from(obj: UserCertificate) -> Self {
        Self {
            ty: Default::default(),
            author: obj.author,
            timestamp: obj.timestamp,
            user_id: obj.user_id,
            human_handle: obj.human_handle.into(),
            public_key: obj.public_key,
            profile: obj.profile.into(),
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

parsec_data!("schema/certif/revoked_user_certificate.json");

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

parsec_data!("schema/certif/device_certificate.json");

impl_verify_and_load_allow_root!(DeviceCertificate);
impl_unsecure_load!(DeviceCertificate);
impl_dump_and_sign!(DeviceCertificate);

impl_transparent_data_format_conversion!(
    DeviceCertificate,
    DeviceCertificateData,
    author,
    timestamp,
    device_id,
    device_label,
    verify_key,
);

/*
 * RealmRoleCertificate
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(into = "RealmRoleCertificateData", from = "RealmRoleCertificateData")]
pub struct RealmRoleCertificate {
    pub author: DeviceID,
    pub timestamp: DateTime,

    pub realm_id: RealmID,
    pub user_id: UserID,
    // Set to None if role removed
    pub role: Option<RealmRole>, // TODO: use a custom type instead
}

impl_verify_and_load_no_root!(RealmRoleCertificate);
impl_unsecure_load!(RealmRoleCertificate);
impl_dump_and_sign!(RealmRoleCertificate);

parsec_data!("schema/certif/realm_role_certificate.json");

impl_transparent_data_format_conversion!(
    RealmRoleCertificate,
    RealmRoleCertificateData,
    author,
    timestamp,
    realm_id,
    user_id,
    role,
);
