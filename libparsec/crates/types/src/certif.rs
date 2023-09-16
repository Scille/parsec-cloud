// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::io::{Read, Write};
use std::sync::Arc;

use bytes::Bytes;
use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use paste::paste;
use serde::{Deserialize, Serialize};
use serde_with::*;

use libparsec_crypto::{
    PublicKey, SequesterPublicKeyDer, SequesterVerifyKeyDer, SigningKey, VerifyKey,
};
use libparsec_serialization_format::parsec_data;

use crate as libparsec_types;
use crate::data_macros::impl_transparent_data_format_conversion;
use crate::{DataError, DataResult};
use crate::{
    DateTime, DeviceID, DeviceLabel, HumanHandle, RealmRole, SequesterServiceID, UserID,
    UserProfile, VlobID,
};

fn load<T>(compressed: &[u8]) -> DataResult<T>
where
    T: for<'a> Deserialize<'a>,
{
    let mut serialized = vec![];
    ZlibDecoder::new(compressed)
        .read_to_end(&mut serialized)
        .map_err(|_| DataError::Compression)?;
    rmp_serde::from_slice(&serialized).map_err(|_| DataError::Serialization)
}

fn verify_and_load<T>(signed: &[u8], author_verify_key: &VerifyKey) -> DataResult<T>
where
    T: for<'a> Deserialize<'a>,
{
    let compressed = author_verify_key
        .verify(signed)
        .map_err(|_| DataError::Signature)?;
    load::<T>(compressed)
}

fn dump<T>(obj: &T) -> Vec<u8>
where
    T: Serialize,
{
    let serialized = ::rmp_serde::to_vec_named(obj).expect("object should be serializable");
    let mut e = ZlibEncoder::new(Vec::new(), flate2::Compression::default());
    e.write_all(&serialized)
        .and_then(|_| e.finish())
        .expect("in-memory buffer should not fail")
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
            return Err(DataError::UnexpectedAuthor {
                expected: Box::new(expected_author_id.clone()),
                got: Some(Box::new(author_id.clone())),
            })
        }
        (CertificateSignerOwned::Root, CertificateSignerRef::User(expected_author_id)) => {
            return Err(DataError::UnexpectedAuthor {
                expected: Box::new(expected_author_id.clone()),
                got: None,
            })
        }
        _ => (),
    }

    Ok(())
}

pub enum UnsecureSkipValidationReason {
    // Certificate must have been validated prior to being added to the local storage,
    // on top of that the certificate are store encrypted so they cannot be tempered.
    // Hence it's safe not to validate the certificate when reading thom from the
    // local storage.
    DataFromLocalStorage,
    #[cfg(feature = "test-fixtures")]
    Test,
}

macro_rules! impl_unsecure_load {
    ($name:ident $( -> $author_type:ty )? ) => {
        paste!{
            #[derive(Debug)]
            pub struct [< Unsecure $name >] {
                signed: Bytes,
                unsecure: $name,
            }
            impl [< Unsecure $name >] {
                $(
                pub fn author(&self) -> &$author_type {
                    &self.unsecure.author
                }
                )?
                pub fn timestamp(&self) -> &DateTime {
                    &self.unsecure.timestamp
                }
                /// Hint is useful to provide of representation the certificate in case
                /// it has failed validation we want to report it in an error message
                pub fn hint(&self) -> String {
                    format!("{:?}", self.unsecure)
                }
                pub fn verify_signature(self, author_verify_key: &VerifyKey) -> Result<($name, Bytes), (Self, DataError)> {
                    match author_verify_key.verify(self.signed.as_ref()) {
                        // Unsecure is now secure \o/
                        Ok(_) => Ok((self.unsecure, self.signed)),
                        Err(_) => Err((self, DataError::Signature)),
                    }
                }
                /// The `reason` field is only there to explain the only places using this function
                /// is not a terrible idea
                pub fn skip_validation(self, _reason: UnsecureSkipValidationReason) -> $name {
                    self.unsecure
                }
            }

            impl $name {
                pub fn unsecure_load(signed: Bytes) -> DataResult<[< Unsecure $name >]> {
                    let (_, compressed) = VerifyKey::unsecure_unwrap(signed.as_ref()).map_err(|_| DataError::Signature)?;
                    let unsecure = load::<$name>(compressed)?;
                    Ok([< Unsecure $name >] {
                        signed,
                        unsecure,
                    })
                }
            }
        }
    };
}

macro_rules! impl_unsecure_dump {
    ($name:ident) => {
        impl $name {
            pub fn unsecure_dump(&self) -> Vec<u8> {
                dump::<Self>(self)
            }
        }
    };
}

macro_rules! impl_dump_and_sign {
    ($name:ident) => {
        impl $name {
            pub fn dump_and_sign(&self, author_signkey: &SigningKey) -> Vec<u8> {
                author_signkey.sign(&self.unsecure_dump())
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

impl_unsecure_load!(UserCertificate -> CertificateSignerOwned);
impl_unsecure_dump!(UserCertificate);
impl_dump_and_sign!(UserCertificate);

impl UserCertificate {
    pub fn verify_and_load(
        signed: &[u8],
        author_verify_key: &VerifyKey,
        expected_author: CertificateSignerRef<'_>,
        expected_user_id: Option<&UserID>,
        expected_human_handle: Option<&HumanHandle>,
    ) -> DataResult<Self> {
        let r = verify_and_load::<Self>(signed, author_verify_key)?;
        check_author_allow_root(&r.author, expected_author)?;

        if let Some(expected_user_id) = expected_user_id {
            if &r.user_id != expected_user_id {
                return Err(DataError::UnexpectedUserID {
                    expected: expected_user_id.clone(),
                    got: r.user_id,
                });
            }
        }

        if let Some(expected_human_handle) = expected_human_handle {
            if let Some(human_handle) = &r.human_handle {
                if human_handle != expected_human_handle {
                    return Err(DataError::UnexpectedHumanHandle {
                        expected: Box::new(expected_human_handle.clone()),
                        got: Box::new(human_handle.clone()),
                    });
                }
            }
        }

        Ok(r)
    }
}

parsec_data!("schema/certif/user_certificate.json5");

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

impl_unsecure_load!(RevokedUserCertificate -> DeviceID);
impl_unsecure_dump!(RevokedUserCertificate);
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
            return Err(DataError::UnexpectedAuthor {
                expected: Box::new(expected_author.clone()),
                got: Some(Box::new(r.author)),
            });
        }

        if let Some(expected_user_id) = expected_user_id {
            if &r.user_id != expected_user_id {
                return Err(DataError::UnexpectedUserID {
                    expected: expected_user_id.clone(),
                    got: r.user_id,
                });
            }
        }

        Ok(r)
    }
}

parsec_data!("schema/certif/revoked_user_certificate.json5");

impl_transparent_data_format_conversion!(
    RevokedUserCertificate,
    RevokedUserCertificateData,
    author,
    timestamp,
    user_id,
);

/*
 * UserUpdateCertificate
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(into = "UserUpdateCertificateData", from = "UserUpdateCertificateData")]
pub struct UserUpdateCertificate {
    pub author: DeviceID,
    pub timestamp: DateTime,

    pub user_id: UserID,
    pub new_profile: UserProfile,
}

impl_unsecure_load!(UserUpdateCertificate -> DeviceID);
impl_unsecure_dump!(UserUpdateCertificate);
impl_dump_and_sign!(UserUpdateCertificate);

impl UserUpdateCertificate {
    pub fn verify_and_load(
        signed: &[u8],
        author_verify_key: &VerifyKey,
        expected_author: &DeviceID,
        expected_user_id: Option<&UserID>,
    ) -> DataResult<Self> {
        let r = verify_and_load::<Self>(signed, author_verify_key)?;

        if &r.author != expected_author {
            return Err(DataError::UnexpectedAuthor {
                expected: Box::new(expected_author.clone()),
                got: Some(Box::new(r.author)),
            });
        }

        if let Some(expected_user_id) = expected_user_id {
            if &r.user_id != expected_user_id {
                return Err(DataError::UnexpectedUserID {
                    expected: expected_user_id.clone(),
                    got: r.user_id,
                });
            }
        }

        Ok(r)
    }
}

parsec_data!("schema/certif/user_update_certificate.json5");

impl_transparent_data_format_conversion!(
    UserUpdateCertificate,
    UserUpdateCertificateData,
    author,
    timestamp,
    user_id,
    new_profile,
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

parsec_data!("schema/certif/device_certificate.json5");

impl_unsecure_load!(DeviceCertificate -> CertificateSignerOwned);
impl_unsecure_dump!(DeviceCertificate);
impl_dump_and_sign!(DeviceCertificate);

impl DeviceCertificate {
    pub fn verify_and_load(
        signed: &[u8],
        author_verify_key: &VerifyKey,
        expected_author: CertificateSignerRef<'_>,
        expected_device_id: Option<&DeviceID>,
    ) -> DataResult<Self> {
        let r = verify_and_load::<Self>(signed, author_verify_key)?;
        check_author_allow_root(&r.author, expected_author)?;

        if let Some(expected_device_id) = expected_device_id {
            if &r.device_id != expected_device_id {
                return Err(DataError::UnexpectedDeviceID {
                    expected: Box::new(expected_device_id.clone()),
                    got: Box::new(r.device_id),
                });
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

    pub realm_id: VlobID,
    pub user_id: UserID,
    // Set to None if role removed
    pub role: Option<RealmRole>, // TODO: use a custom type instead
}

impl_unsecure_load!(RealmRoleCertificate -> CertificateSignerOwned);
impl_unsecure_dump!(RealmRoleCertificate);
impl_dump_and_sign!(RealmRoleCertificate);

impl RealmRoleCertificate {
    pub fn new_root(author: DeviceID, timestamp: DateTime, realm_id: VlobID) -> Self {
        let user_id = author.user_id().to_owned();
        Self {
            author: CertificateSignerOwned::User(author),
            timestamp,
            realm_id,
            user_id,
            role: Some(RealmRole::Owner),
        }
    }

    pub fn verify_and_load(
        signed: &[u8],
        author_verify_key: &VerifyKey,
        expected_author: CertificateSignerRef<'_>,
        expected_realm_id: Option<VlobID>,
        expected_user_id: Option<&UserID>,
    ) -> DataResult<Self> {
        let r = verify_and_load::<Self>(signed, author_verify_key)?;
        check_author_allow_root(&r.author, expected_author)?;

        if let Some(expected_realm_id) = expected_realm_id {
            if r.realm_id != expected_realm_id {
                return Err(DataError::UnexpectedRealmID {
                    expected: expected_realm_id,
                    got: r.realm_id,
                });
            }
        }

        if let Some(expected_user_id) = expected_user_id {
            if &r.user_id != expected_user_id {
                return Err(DataError::UnexpectedUserID {
                    expected: expected_user_id.clone(),
                    got: r.user_id,
                });
            }
        }

        Ok(r)
    }
}

parsec_data!("schema/certif/realm_role_certificate.json5");

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

// `insecure_load` doesn't need to expose the author field here given
// `SequesterAuthorityCertificate` is always signed by the root verify key
impl_unsecure_load!(SequesterAuthorityCertificate);
impl_unsecure_dump!(SequesterAuthorityCertificate);
impl_dump_and_sign!(SequesterAuthorityCertificate);

impl SequesterAuthorityCertificate {
    pub fn verify_and_load(signed: &[u8], author_verify_key: &VerifyKey) -> DataResult<Self> {
        verify_and_load::<Self>(signed, author_verify_key)
    }
}

parsec_data!("schema/certif/sequester_authority_certificate.json5");

impl TryFrom<SequesterAuthorityCertificateData> for SequesterAuthorityCertificate {
    type Error = DataError;
    fn try_from(data: SequesterAuthorityCertificateData) -> Result<Self, Self::Error> {
        if let Some(author) = data.author {
            return Err(DataError::UnexpectedNonRootAuthor(author));
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

impl_unsecure_dump!(SequesterServiceCertificate);

impl SequesterServiceCertificate {
    pub fn dump(&self) -> Vec<u8> {
        dump::<Self>(self)
    }

    pub fn load(buf: &[u8]) -> DataResult<Self> {
        load::<Self>(buf)
    }
}

// Cannot use `impl_insecure_load` macro for `SequesterServiceCertificate` given it is
// signed by the sequester authority, which is a special signature system (RSA, PKI etc.)

#[derive(Debug)]
pub struct UnsecureSequesterServiceCertificate {
    signed: Bytes,
    unsecure: SequesterServiceCertificate,
}

impl UnsecureSequesterServiceCertificate {
    pub fn timestamp(&self) -> &DateTime {
        &self.unsecure.timestamp
    }
    pub fn hint(&self) -> String {
        format!("{:?}", self.unsecure)
    }
    pub fn verify_signature(
        self,
        author_verify_key: &SequesterVerifyKeyDer,
    ) -> Result<(SequesterServiceCertificate, Bytes), (Self, DataError)> {
        match author_verify_key.verify(self.signed.as_ref()) {
            // Unsecure is now secure \o/
            Ok(_) => Ok((self.unsecure, self.signed)),
            Err(_) => Err((self, DataError::Signature)),
        }
    }
    pub fn skip_validation(
        self,
        _reason: UnsecureSkipValidationReason,
    ) -> SequesterServiceCertificate {
        self.unsecure
    }
}

impl SequesterServiceCertificate {
    pub fn unsecure_load(signed: Bytes) -> DataResult<UnsecureSequesterServiceCertificate> {
        let (_, compressed) =
            VerifyKey::unsecure_unwrap(signed.as_ref()).map_err(|_| DataError::Signature)?;
        let unsecure = load::<SequesterServiceCertificate>(compressed)?;
        Ok(UnsecureSequesterServiceCertificate { signed, unsecure })
    }
}

parsec_data!("schema/certif/sequester_service_certificate.json5");

impl_transparent_data_format_conversion!(
    SequesterServiceCertificate,
    SequesterServiceCertificateData,
    timestamp,
    service_id,
    service_label,
    encryption_key_der,
);

/*
 * AnyCertificate
 */

#[derive(Debug, Clone)]
pub enum AnyArcCertificate {
    User(Arc<UserCertificate>),
    Device(Arc<DeviceCertificate>),
    UserUpdate(Arc<UserUpdateCertificate>),
    RevokedUser(Arc<RevokedUserCertificate>),
    RealmRole(Arc<RealmRoleCertificate>),
    SequesterAuthority(Arc<SequesterAuthorityCertificate>),
    SequesterService(Arc<SequesterServiceCertificate>),
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum AnyCertificate {
    User(UserCertificate),
    Device(DeviceCertificate),
    UserUpdate(UserUpdateCertificate),
    RevokedUser(RevokedUserCertificate),
    RealmRole(RealmRoleCertificate),
    SequesterAuthority(SequesterAuthorityCertificate),
    SequesterService(SequesterServiceCertificate),
}

#[derive(Debug)]
pub enum UnsecureAnyCertificate {
    User(UnsecureUserCertificate),
    Device(UnsecureDeviceCertificate),
    UserUpdate(UnsecureUserUpdateCertificate),
    RevokedUser(UnsecureRevokedUserCertificate),
    RealmRole(UnsecureRealmRoleCertificate),
    SequesterAuthority(UnsecureSequesterAuthorityCertificate),
    SequesterService(UnsecureSequesterServiceCertificate),
}

impl AnyCertificate {
    pub fn unsecure_load(signed: Bytes) -> Result<UnsecureAnyCertificate, DataError> {
        let (_, compressed) =
            VerifyKey::unsecure_unwrap(signed.as_ref()).map_err(|_| DataError::Signature)?;
        let unsecure = load::<Self>(compressed)?;
        Ok(match unsecure {
            AnyCertificate::User(unsecure) => {
                UnsecureAnyCertificate::User(UnsecureUserCertificate { signed, unsecure })
            }
            AnyCertificate::Device(unsecure) => {
                UnsecureAnyCertificate::Device(UnsecureDeviceCertificate { signed, unsecure })
            }
            AnyCertificate::RevokedUser(unsecure) => {
                UnsecureAnyCertificate::RevokedUser(UnsecureRevokedUserCertificate {
                    signed,
                    unsecure,
                })
            }
            AnyCertificate::UserUpdate(unsecure) => {
                UnsecureAnyCertificate::UserUpdate(UnsecureUserUpdateCertificate {
                    signed,
                    unsecure,
                })
            }
            AnyCertificate::RealmRole(unsecure) => {
                UnsecureAnyCertificate::RealmRole(UnsecureRealmRoleCertificate { signed, unsecure })
            }
            AnyCertificate::SequesterAuthority(unsecure) => {
                UnsecureAnyCertificate::SequesterAuthority(UnsecureSequesterAuthorityCertificate {
                    signed,
                    unsecure,
                })
            }
            AnyCertificate::SequesterService(unsecure) => {
                UnsecureAnyCertificate::SequesterService(UnsecureSequesterServiceCertificate {
                    signed,
                    unsecure,
                })
            }
        })
    }
}

impl UnsecureAnyCertificate {
    pub fn timestamp(&self) -> &DateTime {
        match self {
            UnsecureAnyCertificate::User(unsecure) => unsecure.timestamp(),
            UnsecureAnyCertificate::Device(unsecure) => unsecure.timestamp(),
            UnsecureAnyCertificate::RevokedUser(unsecure) => unsecure.timestamp(),
            UnsecureAnyCertificate::UserUpdate(unsecure) => unsecure.timestamp(),
            UnsecureAnyCertificate::RealmRole(unsecure) => unsecure.timestamp(),
            UnsecureAnyCertificate::SequesterAuthority(unsecure) => unsecure.timestamp(),
            UnsecureAnyCertificate::SequesterService(unsecure) => unsecure.timestamp(),
        }
    }

    pub fn hint(&self) -> String {
        match self {
            UnsecureAnyCertificate::User(unsecure) => unsecure.hint(),
            UnsecureAnyCertificate::Device(unsecure) => unsecure.hint(),
            UnsecureAnyCertificate::RevokedUser(unsecure) => unsecure.hint(),
            UnsecureAnyCertificate::UserUpdate(unsecure) => unsecure.hint(),
            UnsecureAnyCertificate::RealmRole(unsecure) => unsecure.hint(),
            UnsecureAnyCertificate::SequesterAuthority(unsecure) => unsecure.hint(),
            UnsecureAnyCertificate::SequesterService(unsecure) => unsecure.hint(),
        }
    }
}
