// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use serde::{Deserialize, Serialize};
use serde_with::*;
use std::io::{Read, Write};

use libparsec_crypto::{
    PublicKey, SequesterPublicKeyDer, SequesterVerifyKeyDer, SigningKey, VerifyKey,
};
use libparsec_serialization_format::parsec_data;

use crate as libparsec_types;
use crate::data_macros::impl_transparent_data_format_conversion;
use crate::{DataError, DataResult};
use crate::{
    DateTime, DeviceID, DeviceLabel, HumanHandle, RealmID, RealmRole, SequesterServiceID, UserID,
    UserProfile,
};

fn verify_and_load<T>(signed: &[u8], author_verify_key: &VerifyKey) -> DataResult<T>
where
    T: for<'a> Deserialize<'a>,
{
    let compressed = author_verify_key.verify(signed)?;
    let mut serialized = vec![];
    ZlibDecoder::new(&compressed[..])
        .read_to_end(&mut serialized)
        .map_err(|_| DataError::Compression)?;
    rmp_serde::from_slice(&serialized).map_err(|_| Box::new(DataError::Serialization))
}

fn check_author_allow_root(
    author: &CertificateSignerOwned,
    expected_author: CertificateSignerRef,
) -> DataResult<()> {
    match (author, expected_author) {
        (
            CertificateSignerOwned::User(author_id),
            CertificateSignerRef::User(expected_author_id),
        ) if author_id != expected_author_id => {
            return Err(Box::new(DataError::UnexpectedAuthor {
                expected: expected_author_id.clone(),
                got: Some(author_id.clone()),
            }))
        }
        (CertificateSignerOwned::Root, CertificateSignerRef::User(expected_author_id)) => {
            return Err(Box::new(DataError::UnexpectedAuthor {
                expected: expected_author_id.clone(),
                got: None,
            }))
        }
        _ => (),
    }

    Ok(())
}

macro_rules! impl_unsecure_load {
    ($name:ident) => {
        impl $name {
            pub fn unsecure_load(signed: &[u8]) -> DataResult<$name> {
                let compressed = VerifyKey::unsecure_unwrap(signed).ok_or(DataError::Signature)?;
                let mut serialized = vec![];
                ZlibDecoder::new(&compressed[..])
                    .read_to_end(&mut serialized)
                    .map_err(|_| DataError::Compression)?;
                ::rmp_serde::from_slice(&serialized).map_err(|_| Box::new(DataError::Serialization))
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

impl_unsecure_load!(UserCertificate);
impl_dump_and_sign!(UserCertificate);

impl UserCertificate {
    pub fn verify_and_load<'a>(
        signed: &[u8],
        author_verify_key: &VerifyKey,
        expected_author: CertificateSignerRef<'a>,
        expected_user_id: Option<&UserID>,
        expected_human_handle: Option<&HumanHandle>,
    ) -> DataResult<Self> {
        let r = verify_and_load::<Self>(signed, author_verify_key)?;
        check_author_allow_root(&r.author, expected_author)?;

        if let Some(expected_user_id) = expected_user_id {
            if &r.user_id != expected_user_id {
                return Err(Box::new(DataError::UnexpectedUserID {
                    expected: expected_user_id.clone(),
                    got: r.user_id,
                }));
            }
        }

        if let Some(expected_human_handle) = expected_human_handle {
            if r.human_handle.as_ref() != Some(expected_human_handle) {
                return Err(Box::new(DataError::InvalidHumanHandle));
            }
        }

        Ok(r)
    }
}

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

impl_unsecure_load!(RevokedUserCertificate);
impl_dump_and_sign!(RevokedUserCertificate);

impl RevokedUserCertificate {
    pub fn verify_and_load(
        signed: &[u8],
        author_verify_key: &VerifyKey,
        expected_author: &DeviceID,
        expected_user_id: Option<&UserID>,
    ) -> DataResult<Self> {
        let r = verify_and_load::<Self>(signed, author_verify_key)?;

        if &r.author != expected_author {
            return Err(Box::new(DataError::UnexpectedAuthor {
                expected: expected_author.clone(),
                got: Some(r.author),
            }));
        }

        if let Some(expected_user_id) = expected_user_id {
            if &r.user_id != expected_user_id {
                return Err(Box::new(DataError::UnexpectedUserID {
                    expected: expected_user_id.clone(),
                    got: r.user_id,
                }));
            }
        }

        Ok(r)
    }
}

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

impl_unsecure_load!(DeviceCertificate);
impl_dump_and_sign!(DeviceCertificate);

impl DeviceCertificate {
    pub fn verify_and_load<'a>(
        signed: &[u8],
        author_verify_key: &VerifyKey,
        expected_author: CertificateSignerRef<'a>,
        expected_device_id: Option<&DeviceID>,
    ) -> DataResult<Self> {
        let r = verify_and_load::<Self>(signed, author_verify_key)?;
        check_author_allow_root(&r.author, expected_author)?;

        if let Some(expected_device_id) = expected_device_id {
            if &r.device_id != expected_device_id {
                return Err(Box::new(DataError::UnexpectedDeviceID {
                    expected: expected_device_id.clone(),
                    got: r.device_id,
                }));
            }
        }

        Ok(r)
    }
}

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
    pub author: CertificateSignerOwned,
    pub timestamp: DateTime,

    pub realm_id: RealmID,
    pub user_id: UserID,
    // Set to None if role removed
    pub role: Option<RealmRole>, // TODO: use a custom type instead
}

impl_unsecure_load!(RealmRoleCertificate);
impl_dump_and_sign!(RealmRoleCertificate);

impl RealmRoleCertificate {
    pub fn verify_and_load<'a>(
        signed: &[u8],
        author_verify_key: &VerifyKey,
        expected_author: CertificateSignerRef<'a>,
        expected_realm_id: Option<RealmID>,
        expected_user_id: Option<&UserID>,
    ) -> DataResult<Self> {
        let r = verify_and_load::<Self>(signed, author_verify_key)?;
        check_author_allow_root(&r.author, expected_author)?;

        if let Some(expected_realm_id) = expected_realm_id {
            if r.realm_id != expected_realm_id {
                return Err(Box::new(DataError::UnexpectedRealmID {
                    expected: expected_realm_id,
                    got: r.realm_id,
                }));
            }
        }

        if let Some(expected_user_id) = expected_user_id {
            if &r.user_id != expected_user_id {
                return Err(Box::new(DataError::UnexpectedUserID {
                    expected: expected_user_id.clone(),
                    got: r.user_id,
                }));
            }
        }

        Ok(r)
    }
}

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

/*
 * SequesterAuthorityCertificate
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(
    into = "SequesterAuthorityCertificateData",
    try_from = "SequesterAuthorityCertificateData"
)]
pub struct SequesterAuthorityCertificate {
    pub timestamp: DateTime,
    pub verify_key_der: SequesterVerifyKeyDer,
}

impl_unsecure_load!(SequesterAuthorityCertificate);
impl_dump_and_sign!(SequesterAuthorityCertificate);

impl SequesterAuthorityCertificate {
    pub fn verify_and_load(signed: &[u8], author_verify_key: &VerifyKey) -> DataResult<Self> {
        verify_and_load::<Self>(signed, author_verify_key)
    }
}

parsec_data!("schema/certif/sequester_authority_certificate.json");

impl TryFrom<SequesterAuthorityCertificateData> for SequesterAuthorityCertificate {
    type Error = DataError;
    fn try_from(data: SequesterAuthorityCertificateData) -> Result<Self, Self::Error> {
        if let Some(author) = data.author {
            return Err(DataError::Root(author));
        }

        Ok(Self {
            timestamp: data.timestamp,
            verify_key_der: data.verify_key_der,
        })
    }
}

impl From<SequesterAuthorityCertificate> for SequesterAuthorityCertificateData {
    fn from(obj: SequesterAuthorityCertificate) -> Self {
        Self {
            ty: Default::default(),
            author: None,
            timestamp: obj.timestamp,
            verify_key_der: obj.verify_key_der,
        }
    }
}

/*
 * SequesterServiceCertificate
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(
    into = "SequesterServiceCertificateData",
    from = "SequesterServiceCertificateData"
)]
pub struct SequesterServiceCertificate {
    pub timestamp: DateTime,
    pub service_id: SequesterServiceID,
    pub service_label: String,
    pub encryption_key_der: SequesterPublicKeyDer,
}

impl SequesterServiceCertificate {
    pub fn dump(&self) -> Vec<u8> {
        let serialized = rmp_serde::to_vec_named(self).expect("Unreachable");
        let mut e = ZlibEncoder::new(Vec::new(), flate2::Compression::default());
        e.write_all(&serialized).expect("Unreachable");
        e.finish().expect("Unreachable")
    }

    pub fn load(buf: &[u8]) -> DataResult<Self> {
        let mut serialized = vec![];
        ZlibDecoder::new(buf)
            .read_to_end(&mut serialized)
            .map_err(|_| DataError::Compression)?;
        rmp_serde::from_slice(&serialized).map_err(|_| Box::new(DataError::Serialization))
    }
}

parsec_data!("schema/certif/sequester_service_certificate.json");

impl_transparent_data_format_conversion!(
    SequesterServiceCertificate,
    SequesterServiceCertificateData,
    timestamp,
    service_id,
    service_label,
    encryption_key_der,
);
