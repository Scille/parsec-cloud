// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::collections::{HashMap, HashSet};
use std::num::NonZeroU8;
use std::sync::Arc;

use bytes::Bytes;
use paste::paste;
use serde::{Deserialize, Serialize};
use serde_with::*;

use libparsec_crypto::{
    CryptoError, PublicKey, SecretKey, SequesterPublicKeyDer, SequesterVerifyKeyDer, SigningKey,
    VerifyKey,
};
use libparsec_serialization_format::parsec_data;

use crate::data_macros::impl_transparent_data_format_conversion;
use crate::{self as libparsec_types, IndexInt};
use crate::{
    DataError, DataResult, DateTime, DeviceID, DeviceLabel, HumanHandle, MaybeRedacted, RealmRole,
    SequesterServiceID, UserID, UserProfile, VlobID,
    serialization::{format_v0_dump, format_vx_load},
};

fn check_author_allow_root(
    author: CertificateSigner,
    expected_author: CertificateSigner,
) -> DataResult<()> {
    match (author, expected_author) {
        (CertificateSigner::User(author_id), CertificateSigner::User(expected_author_id))
            if author_id != expected_author_id =>
        {
            return Err(DataError::UnexpectedAuthor {
                expected: expected_author_id,
                got: Some(author_id),
            });
        }
        (CertificateSigner::Root, CertificateSigner::User(expected_author_id)) => {
            return Err(DataError::UnexpectedAuthor {
                expected: expected_author_id,
                got: None,
            });
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

macro_rules! impl_base_load {
    ($name:ident) => {
        impl $name {
            pub fn base_verify_and_load(
                signed: &[u8],
                author_verify_key: &VerifyKey,
            ) -> DataResult<Self> {
                let serialized = author_verify_key
                    .verify(signed)
                    .map_err(|_| DataError::Signature)?;

                let result: Self = $crate::serialization::format_vx_load(serialized)?;
                result.check_data_integrity()?;
                Ok(result)
            }

            pub fn base_unsecure_load(signed: &[u8]) -> DataResult<Self> {
                let (_, serialized) = VerifyKey::unsecure_unwrap(signed.as_ref())
                    .map_err(|_| DataError::Signature)?;
                let unsecure = $crate::serialization::format_vx_load::<$name>(&serialized)?;
                unsecure.check_data_integrity()?;
                Ok(unsecure)
            }
        }
    };
}
pub(super) use impl_base_load;

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
                pub fn author(&self) -> $author_type {
                    self.unsecure.author
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
                    let unsecure = Self::base_unsecure_load(signed.as_ref())?;
                    Ok([< Unsecure $name >] {
                        signed,
                        unsecure,
                    })
                }
            }
        }
    };
}
pub(super) use impl_unsecure_load;

macro_rules! impl_unsecure_dump {
    ($name:ident) => {
        impl $name {
            pub fn unsecure_dump(&self) -> Vec<u8> {
                self.check_data_integrity()
                    .expect("Data integrity check failed");
                $crate::serialization::format_v0_dump(self)
            }
        }
    };
}
pub(super) use impl_unsecure_dump;

macro_rules! impl_dump_and_sign {
    ($name:ident) => {
        impl $name {
            pub fn dump_and_sign(&self, author_signkey: &SigningKey) -> Vec<u8> {
                self.check_data_integrity()
                    .expect("Data integrity check failed");
                author_signkey.sign(&self.unsecure_dump())
            }
        }
    };
}
pub(super) use impl_dump_and_sign;

/*
 * CertificateSigner
 */

/// Signature can be done either by a user (through one of it devices) or
/// by the Root Key when bootstrapping the organization (only the very first
/// user and device certificates are signed this way)
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(into = "Option<DeviceID>", try_from = "Option<DeviceID>")]
pub enum CertificateSigner {
    User(DeviceID),
    Root,
}

impl From<Option<DeviceID>> for CertificateSigner {
    fn from(item: Option<DeviceID>) -> CertificateSigner {
        match item {
            Some(device_id) => CertificateSigner::User(device_id),
            None => CertificateSigner::Root,
        }
    }
}

impl From<CertificateSigner> for Option<DeviceID> {
    fn from(item: CertificateSigner) -> Option<DeviceID> {
        match item {
            CertificateSigner::User(device_id) => Some(device_id),
            CertificateSigner::Root => None,
        }
    }
}

/*
 * UserCertificate
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(into = "UserCertificateData", from = "UserCertificateData")]
pub struct UserCertificate {
    pub author: CertificateSigner,
    pub timestamp: DateTime,

    pub user_id: UserID,
    pub human_handle: MaybeRedacted<HumanHandle>,
    pub public_key: PublicKey,
    pub algorithm: PrivateKeyAlgorithm,
    pub profile: UserProfile,
}

impl_unsecure_load!(UserCertificate -> CertificateSigner);
impl_unsecure_dump!(UserCertificate);
impl_dump_and_sign!(UserCertificate);
impl_base_load!(UserCertificate);

impl UserCertificate {
    /// This structure represents immutable data (as it is created once, signed, and never updated).
    /// Hence this `check_data_integrity` is only used during deserialization (and also as sanity check
    /// right before serialization) and not exposed publicly.
    fn check_data_integrity(&self) -> DataResult<()> {
        Ok(())
    }

    pub fn into_redacted(self) -> Self {
        let human_handle = MaybeRedacted::Redacted(HumanHandle::new_redacted(self.user_id));
        Self {
            author: self.author,
            timestamp: self.timestamp,
            user_id: self.user_id,
            human_handle,
            public_key: self.public_key,
            algorithm: self.algorithm,
            profile: self.profile,
        }
    }

    pub fn verify_and_load(
        signed: &[u8],
        author_verify_key: &VerifyKey,
        expected_author: CertificateSigner,
        expected_user_id: Option<UserID>,
        expected_human_handle: Option<&HumanHandle>,
    ) -> DataResult<Self> {
        let r = Self::base_verify_and_load(signed, author_verify_key)?;
        check_author_allow_root(r.author, expected_author)?;

        if let Some(expected_user_id) = expected_user_id {
            if r.user_id != expected_user_id {
                return Err(DataError::UnexpectedUserID {
                    expected: expected_user_id,
                    got: r.user_id,
                });
            }
        }

        if let Some(expected_human_handle) = expected_human_handle {
            if r.human_handle.as_ref() != expected_human_handle {
                return Err(DataError::UnexpectedHumanHandle {
                    expected: Box::new(expected_human_handle.clone()),
                    got: Box::new(r.human_handle.as_ref().to_owned()),
                });
            }
        }

        Ok(r)
    }
}

parsec_data!("schema/certif/user_certificate.json5");

impl From<UserCertificateData> for UserCertificate {
    fn from(data: UserCertificateData) -> Self {
        let human_handle = match data.human_handle {
            None => MaybeRedacted::Redacted(HumanHandle::new_redacted(data.user_id)),
            Some(human_handle) => MaybeRedacted::Real(human_handle),
        };
        Self {
            author: data.author,
            timestamp: data.timestamp,
            user_id: data.user_id,
            human_handle,
            public_key: data.public_key,
            algorithm: data.algorithm,
            profile: data.profile,
        }
    }
}

impl From<UserCertificate> for UserCertificateData {
    fn from(obj: UserCertificate) -> Self {
        let human_handle = match obj.human_handle {
            MaybeRedacted::Real(human_handle) => Some(human_handle),
            MaybeRedacted::Redacted(_) => None,
        };
        Self {
            ty: Default::default(),
            author: obj.author,
            timestamp: obj.timestamp,
            user_id: obj.user_id,
            human_handle,
            public_key: obj.public_key,
            algorithm: obj.algorithm,
            profile: obj.profile,
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
impl_base_load!(RevokedUserCertificate);

impl RevokedUserCertificate {
    /// This structure represents immutable data (as it is created once, signed, and never updated).
    /// Hence this `check_data_integrity` is only used during deserialization (and also as sanity check
    /// right before serialization) and not exposed publicly.
    fn check_data_integrity(&self) -> DataResult<()> {
        Ok(())
    }

    pub fn verify_and_load(
        signed: &[u8],
        author_verify_key: &VerifyKey,
        expected_author: DeviceID,
        expected_user_id: Option<UserID>,
    ) -> DataResult<Self> {
        let r = Self::base_verify_and_load(signed, author_verify_key)?;

        if r.author != expected_author {
            return Err(DataError::UnexpectedAuthor {
                expected: expected_author,
                got: Some(r.author),
            });
        }

        if let Some(expected_user_id) = expected_user_id {
            if r.user_id != expected_user_id {
                return Err(DataError::UnexpectedUserID {
                    expected: expected_user_id,
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
impl_base_load!(UserUpdateCertificate);

impl UserUpdateCertificate {
    /// This structure represents immutable data (as it is created once, signed, and never updated).
    /// Hence this `check_data_integrity` is only used during deserialization (and also as sanity check
    /// right before serialization) and not exposed publicly.
    fn check_data_integrity(&self) -> DataResult<()> {
        Ok(())
    }

    pub fn verify_and_load(
        signed: &[u8],
        author_verify_key: &VerifyKey,
        expected_author: DeviceID,
        expected_user_id: Option<UserID>,
    ) -> DataResult<Self> {
        let r = Self::base_verify_and_load(signed, author_verify_key)?;

        if r.author != expected_author {
            return Err(DataError::UnexpectedAuthor {
                expected: expected_author,
                got: Some(r.author),
            });
        }

        if let Some(expected_user_id) = expected_user_id {
            if r.user_id != expected_user_id {
                return Err(DataError::UnexpectedUserID {
                    expected: expected_user_id,
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
    pub author: CertificateSigner,
    pub timestamp: DateTime,

    pub purpose: DevicePurpose,
    pub user_id: UserID,
    pub device_id: DeviceID,
    pub device_label: MaybeRedacted<DeviceLabel>,
    pub verify_key: VerifyKey,
    pub algorithm: SigningKeyAlgorithm,
}

parsec_data!("schema/certif/device_certificate.json5");

impl_unsecure_load!(DeviceCertificate -> CertificateSigner);
impl_unsecure_dump!(DeviceCertificate);
impl_dump_and_sign!(DeviceCertificate);
impl_base_load!(DeviceCertificate);

impl DeviceCertificate {
    /// This structure represents immutable data (as it is created once, signed, and never updated).
    /// Hence this `check_data_integrity` is only used during deserialization (and also as sanity check
    /// right before serialization) and not exposed publicly.
    fn check_data_integrity(&self) -> DataResult<()> {
        Ok(())
    }
    pub fn into_redacted(self) -> Self {
        let device_label = MaybeRedacted::Redacted(DeviceLabel::new_redacted(self.device_id));
        Self {
            author: self.author,
            timestamp: self.timestamp,
            purpose: self.purpose,
            user_id: self.user_id,
            device_id: self.device_id,
            device_label,
            verify_key: self.verify_key,
            algorithm: self.algorithm,
        }
    }

    pub fn verify_and_load(
        signed: &[u8],
        author_verify_key: &VerifyKey,
        expected_author: CertificateSigner,
        expected_device_id: Option<DeviceID>,
    ) -> DataResult<Self> {
        let r = Self::base_verify_and_load(signed, author_verify_key)?;
        check_author_allow_root(r.author, expected_author)?;

        if let Some(expected_device_id) = expected_device_id {
            if r.device_id != expected_device_id {
                return Err(DataError::UnexpectedDeviceID {
                    expected: expected_device_id,
                    got: r.device_id,
                });
            }
        }

        Ok(r)
    }
}

impl From<DeviceCertificateData> for DeviceCertificate {
    fn from(data: DeviceCertificateData) -> Self {
        let device_label = match data.device_label {
            None => MaybeRedacted::Redacted(DeviceLabel::new_redacted(data.device_id)),
            Some(device_label) => MaybeRedacted::Real(device_label),
        };
        Self {
            author: data.author,
            timestamp: data.timestamp,
            purpose: data.purpose.unwrap_or(DevicePurpose::Standard),
            user_id: data.user_id,
            device_id: data.device_id,
            device_label,
            verify_key: data.verify_key,
            algorithm: data.algorithm,
        }
    }
}

impl From<DeviceCertificate> for DeviceCertificateData {
    fn from(obj: DeviceCertificate) -> Self {
        let device_label = match obj.device_label {
            MaybeRedacted::Real(device_label) => Some(device_label),
            MaybeRedacted::Redacted(_) => None,
        };
        Self {
            ty: Default::default(),
            author: obj.author,
            timestamp: obj.timestamp,
            purpose: Some(obj.purpose),
            user_id: obj.user_id,
            device_id: obj.device_id,
            device_label,
            verify_key: obj.verify_key,
            algorithm: obj.algorithm,
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

    pub realm_id: VlobID,
    pub user_id: UserID,
    // Set to None if role removed
    pub role: Option<RealmRole>, // TODO: use a custom type instead
}

impl_unsecure_load!(RealmRoleCertificate -> DeviceID);
impl_unsecure_dump!(RealmRoleCertificate);
impl_dump_and_sign!(RealmRoleCertificate);
impl_base_load!(RealmRoleCertificate);

impl RealmRoleCertificate {
    /// This structure represents immutable data (as it is created once, signed, and never updated).
    /// Hence this `check_data_integrity` is only used during deserialization (and also as sanity check
    /// right before serialization) and not exposed publicly.
    fn check_data_integrity(&self) -> DataResult<()> {
        Ok(())
    }

    pub fn new_root(
        author_user_id: UserID,
        author_device_id: DeviceID,
        timestamp: DateTime,
        realm_id: VlobID,
    ) -> Self {
        Self {
            author: author_device_id,
            timestamp,
            realm_id,
            user_id: author_user_id,
            role: Some(RealmRole::Owner),
        }
    }

    pub fn verify_and_load(
        signed: &[u8],
        author_verify_key: &VerifyKey,
        expected_author: DeviceID,
        expected_realm_id: Option<VlobID>,
        expected_user_id: Option<UserID>,
    ) -> DataResult<Self> {
        let r = Self::base_verify_and_load(signed, author_verify_key)?;

        if r.author != expected_author {
            return Err(DataError::UnexpectedAuthor {
                expected: expected_author,
                got: Some(r.author),
            });
        }

        if let Some(expected_realm_id) = expected_realm_id {
            if r.realm_id != expected_realm_id {
                return Err(DataError::UnexpectedRealmID {
                    expected: expected_realm_id,
                    got: r.realm_id,
                });
            }
        }

        if let Some(expected_user_id) = expected_user_id {
            if r.user_id != expected_user_id {
                return Err(DataError::UnexpectedUserID {
                    expected: expected_user_id,
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
 * RealmKeyRotationCertificate
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(
    into = "RealmKeyRotationCertificateData",
    from = "RealmKeyRotationCertificateData"
)]
pub struct RealmKeyRotationCertificate {
    pub author: DeviceID,
    pub timestamp: DateTime,

    pub realm_id: VlobID,
    pub key_index: IndexInt,
    pub encryption_algorithm: SecretKeyAlgorithm,
    pub hash_algorithm: HashAlgorithm,
    pub key_canary: Vec<u8>,
}

impl_unsecure_load!(RealmKeyRotationCertificate -> DeviceID);
impl_unsecure_dump!(RealmKeyRotationCertificate);
impl_dump_and_sign!(RealmKeyRotationCertificate);
impl_base_load!(RealmKeyRotationCertificate);

impl RealmKeyRotationCertificate {
    /// This structure represents immutable data (as it is created once, signed, and never updated).
    /// Hence this `check_data_integrity` is only used during deserialization (and also as sanity check
    /// right before serialization) and not exposed publicly.
    fn check_data_integrity(&self) -> DataResult<()> {
        Ok(())
    }

    pub fn verify_and_load(
        signed: &[u8],
        author_verify_key: &VerifyKey,
        expected_author: DeviceID,
        expected_realm_id: Option<VlobID>,
    ) -> DataResult<Self> {
        let r = Self::base_verify_and_load(signed, author_verify_key)?;

        if r.author != expected_author {
            return Err(DataError::UnexpectedAuthor {
                expected: expected_author,
                got: Some(r.author),
            });
        }

        if let Some(expected_realm_id) = expected_realm_id {
            if r.realm_id != expected_realm_id {
                return Err(DataError::UnexpectedRealmID {
                    expected: expected_realm_id,
                    got: r.realm_id,
                });
            }
        }

        Ok(r)
    }

    pub fn load_key(&self, raw: &[u8]) -> Result<SecretKey, CryptoError> {
        let key: SecretKey = raw.try_into()?;
        // We don't care about the decrypted output (which is supposed to be
        // an empty string anyway), we just validate that the key is the right
        // one given it was able to decrypt the canary.
        let _ = key.decrypt(&self.key_canary)?;
        Ok(key)
    }
}

parsec_data!("schema/certif/realm_key_rotation_certificate.json5");

impl_transparent_data_format_conversion!(
    RealmKeyRotationCertificate,
    RealmKeyRotationCertificateData,
    author,
    timestamp,
    realm_id,
    key_index,
    encryption_algorithm,
    hash_algorithm,
    key_canary,
);

/*
 * RealmNameCertificate
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(into = "RealmNameCertificateData", from = "RealmNameCertificateData")]
pub struct RealmNameCertificate {
    pub author: DeviceID,
    pub timestamp: DateTime,

    pub realm_id: VlobID,
    pub key_index: IndexInt,
    pub encrypted_name: Vec<u8>,
}

impl_unsecure_load!(RealmNameCertificate -> DeviceID);
impl_unsecure_dump!(RealmNameCertificate);
impl_dump_and_sign!(RealmNameCertificate);
impl_base_load!(RealmNameCertificate);

impl RealmNameCertificate {
    /// This structure represents immutable data (as it is created once, signed, and never updated).
    /// Hence this `check_data_integrity` is only used during deserialization (and also as sanity check
    /// right before serialization) and not exposed publicly.
    fn check_data_integrity(&self) -> DataResult<()> {
        Ok(())
    }

    pub fn verify_and_load(
        signed: &[u8],
        author_verify_key: &VerifyKey,
        expected_author: DeviceID,
        expected_realm_id: Option<VlobID>,
    ) -> DataResult<Self> {
        let r = Self::base_verify_and_load(signed, author_verify_key)?;

        if r.author != expected_author {
            return Err(DataError::UnexpectedAuthor {
                expected: expected_author,
                got: Some(r.author),
            });
        }

        if let Some(expected_realm_id) = expected_realm_id {
            if r.realm_id != expected_realm_id {
                return Err(DataError::UnexpectedRealmID {
                    expected: expected_realm_id,
                    got: r.realm_id,
                });
            }
        }

        Ok(r)
    }
}

parsec_data!("schema/certif/realm_name_certificate.json5");

impl_transparent_data_format_conversion!(
    RealmNameCertificate,
    RealmNameCertificateData,
    author,
    timestamp,
    realm_id,
    key_index,
    encrypted_name,
);

/*
 * RealmArchivingCertificate
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(
    into = "RealmArchivingCertificateData",
    from = "RealmArchivingCertificateData"
)]
pub struct RealmArchivingCertificate {
    pub author: DeviceID,
    pub timestamp: DateTime,

    pub realm_id: VlobID,
    pub configuration: RealmArchivingConfiguration,
}

impl_unsecure_load!(RealmArchivingCertificate -> DeviceID);
impl_unsecure_dump!(RealmArchivingCertificate);
impl_dump_and_sign!(RealmArchivingCertificate);
impl_base_load!(RealmArchivingCertificate);

impl RealmArchivingCertificate {
    /// This structure represents immutable data (as it is created once, signed, and never updated).
    /// Hence this `check_data_integrity` is only used during deserialization (and also as sanity check
    /// right before serialization) and not exposed publicly.
    fn check_data_integrity(&self) -> DataResult<()> {
        Ok(())
    }

    pub fn verify_and_load(
        signed: &[u8],
        author_verify_key: &VerifyKey,
        expected_author: DeviceID,
        expected_realm_id: Option<VlobID>,
    ) -> DataResult<Self> {
        let r = Self::base_verify_and_load(signed, author_verify_key)?;

        if r.author != expected_author {
            return Err(DataError::UnexpectedAuthor {
                expected: expected_author,
                got: Some(r.author),
            });
        }

        if let Some(expected_realm_id) = expected_realm_id {
            if r.realm_id != expected_realm_id {
                return Err(DataError::UnexpectedRealmID {
                    expected: expected_realm_id,
                    got: r.realm_id,
                });
            }
        }

        Ok(r)
    }
}

parsec_data!("schema/certif/realm_archiving_certificate.json5");

impl_transparent_data_format_conversion!(
    RealmArchivingCertificate,
    RealmArchivingCertificateData,
    author,
    timestamp,
    realm_id,
    configuration,
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

// `unsecure_load` doesn't need to expose the author field here given
// `SequesterAuthorityCertificate` is always signed by the root verify key
impl_unsecure_load!(SequesterAuthorityCertificate);
impl_unsecure_dump!(SequesterAuthorityCertificate);
impl_dump_and_sign!(SequesterAuthorityCertificate);
impl_base_load!(SequesterAuthorityCertificate);

impl SequesterAuthorityCertificate {
    /// This structure represents immutable data (as it is created once, signed, and never updated).
    /// Hence this `check_data_integrity` is only used during deserialization (and also as sanity check
    /// right before serialization) and not exposed publicly.
    fn check_data_integrity(&self) -> DataResult<()> {
        Ok(())
    }

    pub fn verify_and_load(signed: &[u8], author_verify_key: &VerifyKey) -> DataResult<Self> {
        Self::base_verify_and_load(signed, author_verify_key)
    }
}

parsec_data!("schema/certif/sequester_authority_certificate.json5");

impl_transparent_data_format_conversion!(
    SequesterAuthorityCertificate,
    SequesterAuthorityCertificateData,
    timestamp,
    verify_key_der,
);

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
    /// This structure represents immutable data (as it is created once, signed, and never updated).
    /// Hence this `check_data_integrity` is only used during deserialization (and also as sanity check
    /// right before serialization) and not exposed publicly.
    fn check_data_integrity(&self) -> DataResult<()> {
        Ok(())
    }

    pub fn dump(&self) -> Vec<u8> {
        format_v0_dump(self)
    }

    pub fn load(buf: &[u8]) -> DataResult<Self> {
        format_vx_load(buf)
    }

    pub fn verify_and_load(
        signed: &[u8],
        authority_verify_key: &SequesterVerifyKeyDer,
    ) -> DataResult<Self> {
        let raw = authority_verify_key
            .verify(signed)
            .map_err(|_| DataError::Signature)?;
        Self::load(&raw)
    }
}

// Sequester service certificate is signed by the sequester authority with a special
// signature system (RSA, PKI etc.). Hence it doesn't support unsecure load (which works
// only for signature with regular `SigningKey`).
// That's no big deal however given sequester authority is always provided as the very
// first certificate anyway.

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
 * SequesterRevokedServiceCertificate
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(
    into = "SequesterRevokedServiceCertificateData",
    from = "SequesterRevokedServiceCertificateData"
)]
pub struct SequesterRevokedServiceCertificate {
    pub timestamp: DateTime,
    pub service_id: SequesterServiceID,
}

impl_unsecure_dump!(SequesterRevokedServiceCertificate);

impl SequesterRevokedServiceCertificate {
    /// This structure represents immutable data (as it is created once, signed, and never updated).
    /// Hence this `check_data_integrity` is only used during deserialization (and also as sanity check
    /// right before serialization) and not exposed publicly.
    fn check_data_integrity(&self) -> DataResult<()> {
        Ok(())
    }

    pub fn dump(&self) -> Vec<u8> {
        format_v0_dump(self)
    }

    pub fn load(buf: &[u8]) -> DataResult<Self> {
        format_vx_load(buf)
    }

    pub fn verify_and_load(
        signed: &[u8],
        authority_verify_key: &SequesterVerifyKeyDer,
    ) -> DataResult<Self> {
        let raw = authority_verify_key
            .verify(signed)
            .map_err(|_| DataError::Signature)?;
        Self::load(&raw)
    }
}

// Sequester revoked service certificate is signed by the sequester authority with a special
// signature system (RSA, PKI etc.). Hence it doesn't support unsecure load (which works
// only for signature with regular `SigningKey`).
// That's no big deal however given sequester authority is always provided as the very
// first certificate anyway.

parsec_data!("schema/certif/sequester_revoked_service_certificate.json5");

impl_transparent_data_format_conversion!(
    SequesterRevokedServiceCertificate,
    SequesterRevokedServiceCertificateData,
    timestamp,
    service_id,
);

/*
 * ShamirRecoveryBriefCertificate
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(
    into = "ShamirRecoveryBriefCertificateData",
    from = "ShamirRecoveryBriefCertificateData"
)]
pub struct ShamirRecoveryBriefCertificate {
    pub author: DeviceID,
    pub timestamp: DateTime,

    /// User here must be the one owning the device used as author
    /// (i.e. it is the user to be recovered).
    pub user_id: UserID,
    pub threshold: NonZeroU8,
    /// The total share count across all recipients must be at most 256
    pub per_recipient_shares: HashMap<UserID, NonZeroU8>,
}

impl_unsecure_load!(ShamirRecoveryBriefCertificate -> DeviceID);
impl_unsecure_dump!(ShamirRecoveryBriefCertificate);
impl_dump_and_sign!(ShamirRecoveryBriefCertificate);
impl_base_load!(ShamirRecoveryBriefCertificate);

impl ShamirRecoveryBriefCertificate {
    /// This structure represents immutable data (as it is created once, signed, and never updated).
    /// Hence this `check_data_integrity` is only used during deserialization (and also as sanity check
    /// right before serialization) and not exposed publicly.
    fn check_data_integrity(&self) -> DataResult<()> {
        let total_shares = self
            .per_recipient_shares
            .values()
            .map(|x| x.get() as usize)
            .sum();
        if total_shares > 255 {
            return Err(DataError::DataIntegrity {
                data_type: std::any::type_name::<Self>(),
                invariant: "total_shares <= 255",
            });
        }
        // Note this check also ensures that total_shares > 0 since threshold > 0
        if self.threshold.get() as usize > total_shares {
            return Err(DataError::DataIntegrity {
                data_type: std::any::type_name::<Self>(),
                invariant: "threshold <= total_shares",
            });
        }
        Ok(())
    }

    pub fn verify_and_load(
        signed: &[u8],
        author_verify_key: &VerifyKey,
        expected_author: DeviceID,
    ) -> DataResult<Self> {
        let r = Self::base_verify_and_load(signed, author_verify_key)?;

        if r.author != expected_author {
            return Err(DataError::UnexpectedAuthor {
                expected: expected_author,
                got: Some(r.author),
            });
        }

        Ok(r)
    }
}

parsec_data!("schema/certif/shamir_recovery_brief_certificate.json5");

impl_transparent_data_format_conversion!(
    ShamirRecoveryBriefCertificate,
    ShamirRecoveryBriefCertificateData,
    author,
    timestamp,
    user_id,
    threshold,
    per_recipient_shares,
);

/*
 * ShamirRecoveryShareCertificate
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(
    into = "ShamirRecoveryShareCertificateData",
    from = "ShamirRecoveryShareCertificateData"
)]
pub struct ShamirRecoveryShareCertificate {
    pub author: DeviceID,
    pub timestamp: DateTime,

    /// User here must be the one owning the device used as author
    /// (i.e. it is the user to be recovered).
    pub user_id: UserID,
    pub recipient: UserID,
    pub ciphered_share: Vec<u8>,
}

impl_unsecure_load!(ShamirRecoveryShareCertificate -> DeviceID);
impl_unsecure_dump!(ShamirRecoveryShareCertificate);
impl_dump_and_sign!(ShamirRecoveryShareCertificate);
impl_base_load!(ShamirRecoveryShareCertificate);

impl ShamirRecoveryShareCertificate {
    /// This structure represents immutable data (as it is created once, signed, and never updated).
    /// Hence this `check_data_integrity` is only used during deserialization (and also as sanity check
    /// right before serialization) and not exposed publicly.
    fn check_data_integrity(&self) -> DataResult<()> {
        Ok(())
    }

    pub fn verify_and_load(
        signed: &[u8],
        author_verify_key: &VerifyKey,
        expected_author: DeviceID,
        expected_recipient: Option<UserID>,
    ) -> DataResult<Self> {
        let r = Self::base_verify_and_load(signed, author_verify_key)?;

        if r.author != expected_author {
            return Err(DataError::UnexpectedAuthor {
                expected: expected_author,
                got: Some(r.author),
            });
        }

        if let Some(expected_recipient) = expected_recipient {
            if r.recipient != expected_recipient {
                return Err(DataError::UnexpectedUserID {
                    expected: expected_recipient,
                    got: r.recipient,
                });
            }
        }

        Ok(r)
    }
}

parsec_data!("schema/certif/shamir_recovery_share_certificate.json5");

impl_transparent_data_format_conversion!(
    ShamirRecoveryShareCertificate,
    ShamirRecoveryShareCertificateData,
    author,
    timestamp,
    user_id,
    recipient,
    ciphered_share,
);

/*
 * ShamirRecoveryDeletionCertificate
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(
    into = "ShamirRecoveryDeletionCertificateData",
    from = "ShamirRecoveryDeletionCertificateData"
)]
pub struct ShamirRecoveryDeletionCertificate {
    pub author: DeviceID,
    pub timestamp: DateTime,

    /// The timestamp of the shamir recovery this certificate removes.
    /// Given the timestamp are strictly growing, unique identification
    /// can be done with the couple user_id + setup_timestamp.
    pub setup_to_delete_timestamp: DateTime,
    /// User here must be the one owning the device used as author
    /// (i.e. a user can only remove its own Shamir recovery)
    pub setup_to_delete_user_id: UserID,
    pub share_recipients: HashSet<UserID>,
}

impl_unsecure_load!(ShamirRecoveryDeletionCertificate -> DeviceID);
impl_unsecure_dump!(ShamirRecoveryDeletionCertificate);
impl_dump_and_sign!(ShamirRecoveryDeletionCertificate);
impl_base_load!(ShamirRecoveryDeletionCertificate);

impl ShamirRecoveryDeletionCertificate {
    /// This structure represents immutable data (as it is created once, signed, and never updated).
    /// Hence this `check_data_integrity` is only used during deserialization (and also as sanity check
    /// right before serialization) and not exposed publicly.
    fn check_data_integrity(&self) -> DataResult<()> {
        Ok(())
    }

    pub fn verify_and_load(
        signed: &[u8],
        author_verify_key: &VerifyKey,
        expected_author: DeviceID,
    ) -> DataResult<Self> {
        let r = Self::base_verify_and_load(signed, author_verify_key)?;

        if r.author != expected_author {
            return Err(DataError::UnexpectedAuthor {
                expected: expected_author,
                got: Some(r.author),
            });
        }

        Ok(r)
    }
}

parsec_data!("schema/certif/shamir_recovery_deletion_certificate.json5");

impl_transparent_data_format_conversion!(
    ShamirRecoveryDeletionCertificate,
    ShamirRecoveryDeletionCertificateData,
    author,
    timestamp,
    setup_to_delete_timestamp,
    setup_to_delete_user_id,
    share_recipients,
);

/*
 * AnyCertificate
 */

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AnyArcCertificate {
    User(Arc<UserCertificate>),
    Device(Arc<DeviceCertificate>),
    UserUpdate(Arc<UserUpdateCertificate>),
    RevokedUser(Arc<RevokedUserCertificate>),
    RealmRole(Arc<RealmRoleCertificate>),
    RealmName(Arc<RealmNameCertificate>),
    RealmArchiving(Arc<RealmArchivingCertificate>),
    RealmKeyRotation(Arc<RealmKeyRotationCertificate>),
    ShamirRecoveryBrief(Arc<ShamirRecoveryBriefCertificate>),
    ShamirRecoveryShare(Arc<ShamirRecoveryShareCertificate>),
    ShamirRecoveryDeletion(Arc<ShamirRecoveryDeletionCertificate>),
    SequesterAuthority(Arc<SequesterAuthorityCertificate>),
    SequesterService(Arc<SequesterServiceCertificate>),
    SequesterRevokedService(Arc<SequesterRevokedServiceCertificate>),
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum AnyCertificate {
    User(UserCertificate),
    Device(DeviceCertificate),
    UserUpdate(UserUpdateCertificate),
    RevokedUser(RevokedUserCertificate),
    RealmRole(RealmRoleCertificate),
    RealmName(RealmNameCertificate),
    RealmArchiving(RealmArchivingCertificate),
    RealmKeyRotation(RealmKeyRotationCertificate),
    ShamirRecoveryBrief(ShamirRecoveryBriefCertificate),
    ShamirRecoveryShare(ShamirRecoveryShareCertificate),
    ShamirRecoveryDeletion(ShamirRecoveryDeletionCertificate),
    SequesterAuthority(SequesterAuthorityCertificate),
    SequesterService(SequesterServiceCertificate),
    SequesterRevokedService(SequesterRevokedServiceCertificate),
}

#[derive(Debug)]
pub enum UnsecureAnyCertificate {
    User(UnsecureUserCertificate),
    Device(UnsecureDeviceCertificate),
    UserUpdate(UnsecureUserUpdateCertificate),
    RevokedUser(UnsecureRevokedUserCertificate),
    RealmRole(UnsecureRealmRoleCertificate),
    RealmName(UnsecureRealmNameCertificate),
    RealmArchiving(UnsecureRealmArchivingCertificate),
    RealmKeyRotation(UnsecureRealmKeyRotationCertificate),
    ShamirRecoveryBrief(UnsecureShamirRecoveryBriefCertificate),
    ShamirRecoveryShare(UnsecureShamirRecoveryShareCertificate),
    ShamirRecoveryDeletion(UnsecureShamirRecoveryDeletionCertificate),
    SequesterAuthority(UnsecureSequesterAuthorityCertificate),
}

impl_base_load!(AnyCertificate);

impl AnyCertificate {
    /// This structure represents immutable data (as it is created once, signed, and never updated).
    /// Hence this `check_data_integrity` is only used during deserialization (and also as sanity check
    /// right before serialization) and not exposed publicly.
    fn check_data_integrity(&self) -> DataResult<()> {
        match self {
            AnyCertificate::User(c) => c.check_data_integrity(),
            AnyCertificate::Device(c) => c.check_data_integrity(),
            AnyCertificate::UserUpdate(c) => c.check_data_integrity(),
            AnyCertificate::RevokedUser(c) => c.check_data_integrity(),
            AnyCertificate::RealmRole(c) => c.check_data_integrity(),
            AnyCertificate::RealmName(c) => c.check_data_integrity(),
            AnyCertificate::RealmArchiving(c) => c.check_data_integrity(),
            AnyCertificate::RealmKeyRotation(c) => c.check_data_integrity(),
            AnyCertificate::ShamirRecoveryBrief(c) => c.check_data_integrity(),
            AnyCertificate::ShamirRecoveryShare(c) => c.check_data_integrity(),
            AnyCertificate::ShamirRecoveryDeletion(c) => c.check_data_integrity(),
            AnyCertificate::SequesterAuthority(c) => c.check_data_integrity(),
            AnyCertificate::SequesterService(c) => c.check_data_integrity(),
            AnyCertificate::SequesterRevokedService(c) => c.check_data_integrity(),
        }
    }

    pub fn unsecure_load(signed: Bytes) -> Result<UnsecureAnyCertificate, DataError> {
        let unsecure = Self::base_unsecure_load(&signed)?;
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
            AnyCertificate::RealmName(unsecure) => {
                UnsecureAnyCertificate::RealmName(UnsecureRealmNameCertificate { signed, unsecure })
            }
            AnyCertificate::RealmArchiving(unsecure) => {
                UnsecureAnyCertificate::RealmArchiving(UnsecureRealmArchivingCertificate {
                    signed,
                    unsecure,
                })
            }
            AnyCertificate::RealmKeyRotation(unsecure) => {
                UnsecureAnyCertificate::RealmKeyRotation(UnsecureRealmKeyRotationCertificate {
                    signed,
                    unsecure,
                })
            }
            AnyCertificate::ShamirRecoveryBrief(unsecure) => {
                UnsecureAnyCertificate::ShamirRecoveryBrief(
                    UnsecureShamirRecoveryBriefCertificate { signed, unsecure },
                )
            }
            AnyCertificate::ShamirRecoveryShare(unsecure) => {
                UnsecureAnyCertificate::ShamirRecoveryShare(
                    UnsecureShamirRecoveryShareCertificate { signed, unsecure },
                )
            }
            AnyCertificate::ShamirRecoveryDeletion(unsecure) => {
                UnsecureAnyCertificate::ShamirRecoveryDeletion(
                    UnsecureShamirRecoveryDeletionCertificate { signed, unsecure },
                )
            }
            AnyCertificate::SequesterAuthority(unsecure) => {
                UnsecureAnyCertificate::SequesterAuthority(UnsecureSequesterAuthorityCertificate {
                    signed,
                    unsecure,
                })
            }
            // Sequester service & revoked service certificates are signed by the sequester
            // authority, this implies two things:
            // - We *always* know the author of the signature, and hence don't need
            //   `unsecure_load` (since it goal is to peek inside the certificate before
            //   it is properly validated in order to get the author).
            // - Those certificates are signed in a different way than the other certificates,
            //   so we can only end up here by (bad) luck if the two signature algorithms
            //   happened to have a similar layout.
            //
            // In any way, we should never end up here, and if we do then it's a
            // "bullshit-in, bullshit-out" case...
            AnyCertificate::SequesterService(_) | AnyCertificate::SequesterRevokedService(_) => {
                panic!("signed by sequester authority");
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
            UnsecureAnyCertificate::RealmName(unsecure) => unsecure.timestamp(),
            UnsecureAnyCertificate::RealmArchiving(unsecure) => unsecure.timestamp(),
            UnsecureAnyCertificate::RealmKeyRotation(unsecure) => unsecure.timestamp(),
            UnsecureAnyCertificate::ShamirRecoveryBrief(unsecure) => unsecure.timestamp(),
            UnsecureAnyCertificate::ShamirRecoveryShare(unsecure) => unsecure.timestamp(),
            UnsecureAnyCertificate::ShamirRecoveryDeletion(unsecure) => unsecure.timestamp(),
            UnsecureAnyCertificate::SequesterAuthority(unsecure) => unsecure.timestamp(),
        }
    }

    pub fn hint(&self) -> String {
        match self {
            UnsecureAnyCertificate::User(unsecure) => unsecure.hint(),
            UnsecureAnyCertificate::Device(unsecure) => unsecure.hint(),
            UnsecureAnyCertificate::RevokedUser(unsecure) => unsecure.hint(),
            UnsecureAnyCertificate::UserUpdate(unsecure) => unsecure.hint(),
            UnsecureAnyCertificate::RealmRole(unsecure) => unsecure.hint(),
            UnsecureAnyCertificate::RealmName(unsecure) => unsecure.hint(),
            UnsecureAnyCertificate::RealmArchiving(unsecure) => unsecure.hint(),
            UnsecureAnyCertificate::RealmKeyRotation(unsecure) => unsecure.hint(),
            UnsecureAnyCertificate::ShamirRecoveryBrief(unsecure) => unsecure.hint(),
            UnsecureAnyCertificate::ShamirRecoveryShare(unsecure) => unsecure.hint(),
            UnsecureAnyCertificate::ShamirRecoveryDeletion(unsecure) => unsecure.hint(),
            UnsecureAnyCertificate::SequesterAuthority(unsecure) => unsecure.hint(),
        }
    }
}

/*
 * CommonTopicCertificate
 */

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommonTopicArcCertificate {
    User(Arc<UserCertificate>),
    Device(Arc<DeviceCertificate>),
    UserUpdate(Arc<UserUpdateCertificate>),
    RevokedUser(Arc<RevokedUserCertificate>),
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum CommonTopicCertificate {
    User(UserCertificate),
    Device(DeviceCertificate),
    UserUpdate(UserUpdateCertificate),
    RevokedUser(RevokedUserCertificate),
}

#[derive(Debug)]
pub enum UnsecureCommonTopicCertificate {
    User(UnsecureUserCertificate),
    Device(UnsecureDeviceCertificate),
    UserUpdate(UnsecureUserUpdateCertificate),
    RevokedUser(UnsecureRevokedUserCertificate),
}

impl_base_load!(CommonTopicCertificate);

impl CommonTopicCertificate {
    /// This structure represents immutable data (as it is created once, signed, and never updated).
    /// Hence this `check_data_integrity` is only used during deserialization (and also as sanity check
    /// right before serialization) and not exposed publicly.
    fn check_data_integrity(&self) -> DataResult<()> {
        match self {
            CommonTopicCertificate::User(c) => c.check_data_integrity(),
            CommonTopicCertificate::Device(c) => c.check_data_integrity(),
            CommonTopicCertificate::RevokedUser(c) => c.check_data_integrity(),
            CommonTopicCertificate::UserUpdate(c) => c.check_data_integrity(),
        }
    }

    pub fn unsecure_load(signed: Bytes) -> Result<UnsecureCommonTopicCertificate, DataError> {
        let unsecure = Self::base_unsecure_load(&signed)?;
        Ok(match unsecure {
            CommonTopicCertificate::User(unsecure) => {
                UnsecureCommonTopicCertificate::User(UnsecureUserCertificate { signed, unsecure })
            }
            CommonTopicCertificate::Device(unsecure) => {
                UnsecureCommonTopicCertificate::Device(UnsecureDeviceCertificate {
                    signed,
                    unsecure,
                })
            }
            CommonTopicCertificate::RevokedUser(unsecure) => {
                UnsecureCommonTopicCertificate::RevokedUser(UnsecureRevokedUserCertificate {
                    signed,
                    unsecure,
                })
            }
            CommonTopicCertificate::UserUpdate(unsecure) => {
                UnsecureCommonTopicCertificate::UserUpdate(UnsecureUserUpdateCertificate {
                    signed,
                    unsecure,
                })
            }
        })
    }
}

impl UnsecureCommonTopicCertificate {
    pub fn timestamp(&self) -> &DateTime {
        match self {
            UnsecureCommonTopicCertificate::User(unsecure) => unsecure.timestamp(),
            UnsecureCommonTopicCertificate::Device(unsecure) => unsecure.timestamp(),
            UnsecureCommonTopicCertificate::RevokedUser(unsecure) => unsecure.timestamp(),
            UnsecureCommonTopicCertificate::UserUpdate(unsecure) => unsecure.timestamp(),
        }
    }

    pub fn hint(&self) -> String {
        match self {
            UnsecureCommonTopicCertificate::User(unsecure) => unsecure.hint(),
            UnsecureCommonTopicCertificate::Device(unsecure) => unsecure.hint(),
            UnsecureCommonTopicCertificate::RevokedUser(unsecure) => unsecure.hint(),
            UnsecureCommonTopicCertificate::UserUpdate(unsecure) => unsecure.hint(),
        }
    }

    pub fn skip_validation(
        self,
        reason: UnsecureSkipValidationReason,
    ) -> CommonTopicArcCertificate {
        match self {
            UnsecureCommonTopicCertificate::User(unsecure) => {
                CommonTopicArcCertificate::User(Arc::new(unsecure.skip_validation(reason)))
            }
            UnsecureCommonTopicCertificate::Device(unsecure) => {
                CommonTopicArcCertificate::Device(Arc::new(unsecure.skip_validation(reason)))
            }
            UnsecureCommonTopicCertificate::RevokedUser(unsecure) => {
                CommonTopicArcCertificate::RevokedUser(Arc::new(unsecure.skip_validation(reason)))
            }
            UnsecureCommonTopicCertificate::UserUpdate(unsecure) => {
                CommonTopicArcCertificate::UserUpdate(Arc::new(unsecure.skip_validation(reason)))
            }
        }
    }
}

/*
 * SequesterTopicCertificate
 */

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SequesterTopicArcCertificate {
    SequesterAuthority(Arc<SequesterAuthorityCertificate>),
    SequesterService(Arc<SequesterServiceCertificate>),
    SequesterRevokedService(Arc<SequesterRevokedServiceCertificate>),
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum SequesterTopicCertificate {
    SequesterAuthority(SequesterAuthorityCertificate),
    SequesterService(SequesterServiceCertificate),
    SequesterRevokedService(SequesterRevokedServiceCertificate),
}

impl SequesterTopicCertificate {
    /// Sequester authority is the very first certificate that should be provided. After
    /// that, all subsequent certificates in this topic must be signed by the authority.
    /// Hence only the authority requires to support the unsecure load.
    pub fn unsecure_load_authority(
        signed: Bytes,
    ) -> Result<UnsecureSequesterAuthorityCertificate, DataError> {
        let (_, serialized) =
            VerifyKey::unsecure_unwrap(signed.as_ref()).map_err(|_| DataError::Signature)?;
        let unsecure = format_vx_load::<SequesterAuthorityCertificate>(serialized)?;
        Ok(UnsecureSequesterAuthorityCertificate { signed, unsecure })
    }
}

/*
 * RealmTopicCertificate
 */

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RealmTopicArcCertificate {
    RealmRole(Arc<RealmRoleCertificate>),
    RealmName(Arc<RealmNameCertificate>),
    RealmKeyRotation(Arc<RealmKeyRotationCertificate>),
    RealmArchiving(Arc<RealmArchivingCertificate>),
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum RealmTopicCertificate {
    RealmRole(RealmRoleCertificate),
    RealmName(RealmNameCertificate),
    RealmKeyRotation(RealmKeyRotationCertificate),
    RealmArchiving(RealmArchivingCertificate),
}

#[derive(Debug)]
pub enum UnsecureRealmTopicCertificate {
    RealmRole(UnsecureRealmRoleCertificate),
    RealmName(UnsecureRealmNameCertificate),
    RealmKeyRotation(UnsecureRealmKeyRotationCertificate),
    RealmArchiving(UnsecureRealmArchivingCertificate),
}

impl_base_load!(RealmTopicCertificate);

impl RealmTopicCertificate {
    /// This structure represents immutable data (as it is created once, signed, and never updated).
    /// Hence this `check_data_integrity` is only used during deserialization (and also as sanity check
    /// right before serialization) and not exposed publicly.
    fn check_data_integrity(&self) -> DataResult<()> {
        match self {
            RealmTopicCertificate::RealmRole(c) => c.check_data_integrity(),
            RealmTopicCertificate::RealmName(c) => c.check_data_integrity(),
            RealmTopicCertificate::RealmKeyRotation(c) => c.check_data_integrity(),
            RealmTopicCertificate::RealmArchiving(c) => c.check_data_integrity(),
        }
    }

    pub fn unsecure_load(signed: Bytes) -> Result<UnsecureRealmTopicCertificate, DataError> {
        let unsecure = Self::base_unsecure_load(&signed)?;
        Ok(match unsecure {
            RealmTopicCertificate::RealmRole(unsecure) => {
                UnsecureRealmTopicCertificate::RealmRole(UnsecureRealmRoleCertificate {
                    signed,
                    unsecure,
                })
            }
            RealmTopicCertificate::RealmName(unsecure) => {
                UnsecureRealmTopicCertificate::RealmName(UnsecureRealmNameCertificate {
                    signed,
                    unsecure,
                })
            }
            RealmTopicCertificate::RealmKeyRotation(unsecure) => {
                UnsecureRealmTopicCertificate::RealmKeyRotation(
                    UnsecureRealmKeyRotationCertificate { signed, unsecure },
                )
            }
            RealmTopicCertificate::RealmArchiving(unsecure) => {
                UnsecureRealmTopicCertificate::RealmArchiving(UnsecureRealmArchivingCertificate {
                    signed,
                    unsecure,
                })
            }
        })
    }
}

impl UnsecureRealmTopicCertificate {
    pub fn timestamp(&self) -> &DateTime {
        match self {
            UnsecureRealmTopicCertificate::RealmRole(unsecure) => unsecure.timestamp(),
            UnsecureRealmTopicCertificate::RealmName(unsecure) => unsecure.timestamp(),
            UnsecureRealmTopicCertificate::RealmKeyRotation(unsecure) => unsecure.timestamp(),
            UnsecureRealmTopicCertificate::RealmArchiving(unsecure) => unsecure.timestamp(),
        }
    }

    pub fn hint(&self) -> String {
        match self {
            UnsecureRealmTopicCertificate::RealmRole(unsecure) => unsecure.hint(),
            UnsecureRealmTopicCertificate::RealmName(unsecure) => unsecure.hint(),
            UnsecureRealmTopicCertificate::RealmKeyRotation(unsecure) => unsecure.hint(),
            UnsecureRealmTopicCertificate::RealmArchiving(unsecure) => unsecure.hint(),
        }
    }
}

/*
 * ShamirRecoveryTopicCertificate
 */

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ShamirRecoveryTopicArcCertificate {
    ShamirRecoveryShare(Arc<ShamirRecoveryShareCertificate>),
    ShamirRecoveryBrief(Arc<ShamirRecoveryBriefCertificate>),
    ShamirRecoveryDeletion(Arc<ShamirRecoveryDeletionCertificate>),
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum ShamirRecoveryTopicCertificate {
    ShamirRecoveryShare(ShamirRecoveryShareCertificate),
    ShamirRecoveryBrief(ShamirRecoveryBriefCertificate),
    ShamirRecoveryDeletion(ShamirRecoveryDeletionCertificate),
}

#[derive(Debug)]
pub enum UnsecureShamirRecoveryTopicCertificate {
    ShamirRecoveryShare(UnsecureShamirRecoveryShareCertificate),
    ShamirRecoveryBrief(UnsecureShamirRecoveryBriefCertificate),
    ShamirRecoveryDeletion(UnsecureShamirRecoveryDeletionCertificate),
}

impl_base_load!(ShamirRecoveryTopicCertificate);

impl ShamirRecoveryTopicCertificate {
    /// This structure represents immutable data (as it is created once, signed, and never updated).
    /// Hence this `check_data_integrity` is only used during deserialization (and also as sanity check
    /// right before serialization) and not exposed publicly.
    fn check_data_integrity(&self) -> DataResult<()> {
        match self {
            ShamirRecoveryTopicCertificate::ShamirRecoveryShare(c) => c.check_data_integrity(),
            ShamirRecoveryTopicCertificate::ShamirRecoveryBrief(c) => c.check_data_integrity(),
            ShamirRecoveryTopicCertificate::ShamirRecoveryDeletion(c) => c.check_data_integrity(),
        }
    }

    pub fn unsecure_load(
        signed: Bytes,
    ) -> Result<UnsecureShamirRecoveryTopicCertificate, DataError> {
        let unsecure = Self::base_unsecure_load(&signed)?;
        Ok(match unsecure {
            ShamirRecoveryTopicCertificate::ShamirRecoveryShare(unsecure) => {
                UnsecureShamirRecoveryTopicCertificate::ShamirRecoveryShare(
                    UnsecureShamirRecoveryShareCertificate { signed, unsecure },
                )
            }
            ShamirRecoveryTopicCertificate::ShamirRecoveryBrief(unsecure) => {
                UnsecureShamirRecoveryTopicCertificate::ShamirRecoveryBrief(
                    UnsecureShamirRecoveryBriefCertificate { signed, unsecure },
                )
            }
            ShamirRecoveryTopicCertificate::ShamirRecoveryDeletion(unsecure) => {
                UnsecureShamirRecoveryTopicCertificate::ShamirRecoveryDeletion(
                    UnsecureShamirRecoveryDeletionCertificate { signed, unsecure },
                )
            }
        })
    }
}

impl UnsecureShamirRecoveryTopicCertificate {
    pub fn timestamp(&self) -> &DateTime {
        match self {
            UnsecureShamirRecoveryTopicCertificate::ShamirRecoveryShare(unsecure) => {
                unsecure.timestamp()
            }
            UnsecureShamirRecoveryTopicCertificate::ShamirRecoveryBrief(unsecure) => {
                unsecure.timestamp()
            }
            UnsecureShamirRecoveryTopicCertificate::ShamirRecoveryDeletion(unsecure) => {
                unsecure.timestamp()
            }
        }
    }

    pub fn hint(&self) -> String {
        match self {
            UnsecureShamirRecoveryTopicCertificate::ShamirRecoveryShare(unsecure) => {
                unsecure.hint()
            }
            UnsecureShamirRecoveryTopicCertificate::ShamirRecoveryBrief(unsecure) => {
                unsecure.hint()
            }
            UnsecureShamirRecoveryTopicCertificate::ShamirRecoveryDeletion(unsecure) => {
                unsecure.hint()
            }
        }
    }
}

#[cfg(test)]
#[path = "../tests/unit/certif.rs"]
#[allow(clippy::unwrap_used)]
mod tests;
