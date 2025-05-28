// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use bytes::Bytes;
use libparsec_crypto::SecretKey;
use rand::Rng;
use serde::{Deserialize, Serialize};
use serde_with::*;
use thiserror::Error;

use libparsec_serialization_format::parsec_data;

use crate::{
    self as libparsec_types,
    data_macros::impl_transparent_data_format_conversion,
    serialization::{format_v0_dump, format_vx_load},
    DataError, DeviceID, OrganizationID, UserID,
};

/*
 * Helpers
 */

macro_rules! impl_dump {
    ($name:ident) => {
        impl $name {
            pub fn dump(&self) -> Vec<u8> {
                format_v0_dump(&self)
            }
        }
    };
}

macro_rules! impl_dump_and_encrypt {
    ($name:ident) => {
        impl $name {
            pub fn dump_and_encrypt(&self, key: &::libparsec_crypto::SecretKey) -> Vec<u8> {
                let serialized = format_v0_dump(&self);
                key.encrypt(&serialized)
            }
        }
    };
}

macro_rules! impl_load {
    ($name:ident) => {
        impl $name {
            pub fn load(serialized: &[u8]) -> Result<$name, DataError> {
                format_vx_load(&serialized)
            }
        }
    };
}

macro_rules! impl_decrypt_and_load {
    ($name:ident) => {
        impl $name {
            pub fn decrypt_and_load(
                encrypted: &[u8],
                key: &::libparsec_crypto::SecretKey,
            ) -> Result<$name, DataError> {
                let serialized = key.decrypt(encrypted).map_err(|_| DataError::Decryption)?;
                format_vx_load(&serialized)
            }
        }
    };
}

/*
 * ValidationCode
 */

#[derive(Error, Debug)]
pub enum ValidationCodeParseError {
    #[error("Bad size")]
    BadSize,
    #[error("Not base32")]
    NotBase32,
}

const VALIDATION_CODE_CHARS: &[u8; 32] = crate::BASE32_ALPHABET;
const VALIDATION_CODE_SIZE: usize = 6;

/// A code, typically transmitted by email, used to have a two factor validation
/// on an operation (e.g. delete/reset account).
///
/// The code format is a 6 characters string using base32 (i.e. RFC 4648).
/// Example: `AD3FXJ`
#[serde_as]
#[derive(Clone, Serialize, DeserializeFromStr, PartialEq, Eq)]
#[serde(try_from = "&str", into = "String")]
#[non_exhaustive] // Prevent initialization without going through the factory
pub struct ValidationCode([u8; VALIDATION_CODE_SIZE]);

impl Default for ValidationCode {
    fn default() -> Self {
        let mut bytes = [0u8; VALIDATION_CODE_SIZE];
        for c in bytes.iter_mut() {
            let r = rand::thread_rng().gen_range(0..32);
            *c = VALIDATION_CODE_CHARS[r];
        }
        Self(bytes)
    }
}

impl std::fmt::Debug for ValidationCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("ValidationCode")
            .field(&self.as_ref())
            .finish()
    }
}

impl std::str::FromStr for ValidationCode {
    type Err = ValidationCodeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bytes: [u8; VALIDATION_CODE_SIZE] = s
            .as_bytes()
            .try_into()
            .map_err(|_| ValidationCodeParseError::BadSize)?;
        for b in bytes {
            if !VALIDATION_CODE_CHARS.contains(&b) {
                return Err(ValidationCodeParseError::NotBase32);
            }
        }

        Ok(Self(bytes))
    }
}

impl AsRef<str> for ValidationCode {
    fn as_ref(&self) -> &str {
        std::str::from_utf8(&self.0).expect("uses a subset of UTF8")
    }
}

impl From<ValidationCode> for String {
    fn from(value: ValidationCode) -> Self {
        value.as_ref().to_owned()
    }
}

/*
 * AccountVaultItem
 */

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum AccountVaultItem {
    WebLocalDeviceKey(AccountVaultItemWebLocalDeviceKey),
    RegistrationDevice(AccountVaultItemRegistrationDevice),
}

impl_load!(AccountVaultItem);

/*
 * AccountVaultItemWebLocalDeviceKey
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(
    into = "AccountVaultItemWebLocalDeviceKeyData",
    from = "AccountVaultItemWebLocalDeviceKeyData"
)]
pub struct AccountVaultItemWebLocalDeviceKey {
    pub organization_id: OrganizationID,
    pub device_id: DeviceID,
    /// `SecretKey` encrypted by the vault key.
    /// This key is itself used to decrypt the `LocalDevice` stored on
    /// the web client's storage
    pub encrypted_data: Bytes,
}

parsec_data!("schema/account/account_vault_item_web_local_device_key.json5");

impl_transparent_data_format_conversion!(
    AccountVaultItemWebLocalDeviceKey,
    AccountVaultItemWebLocalDeviceKeyData,
    organization_id,
    device_id,
    encrypted_data,
);

impl_dump!(AccountVaultItemWebLocalDeviceKey);

/*
 * AccountVaultItemRegistrationDevice
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(
    into = "AccountVaultItemRegistrationDeviceData",
    from = "AccountVaultItemRegistrationDeviceData"
)]
pub struct AccountVaultItemRegistrationDevice {
    pub organization_id: OrganizationID,
    pub user_id: UserID,
    /// `LocalDevice` encrypted by the vault key
    pub encrypted_data: Bytes,
}

parsec_data!("schema/account/account_vault_item_registration_device.json5");

impl_transparent_data_format_conversion!(
    AccountVaultItemRegistrationDevice,
    AccountVaultItemRegistrationDeviceData,
    organization_id,
    user_id,
    encrypted_data,
);

impl_dump!(AccountVaultItemRegistrationDevice);

/*
 * AccountVaultKeyAccess
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(into = "AccountVaultKeyAccessData", from = "AccountVaultKeyAccessData")]
pub struct AccountVaultKeyAccess {
    pub vault_key: SecretKey,
}

parsec_data!("schema/account/account_vault_key_access.json5");

impl_transparent_data_format_conversion!(
    AccountVaultKeyAccess,
    AccountVaultKeyAccessData,
    vault_key,
);

impl_dump_and_encrypt!(AccountVaultKeyAccess);
impl_decrypt_and_load!(AccountVaultKeyAccess);

#[cfg(test)]
#[path = "../tests/unit/account.rs"]
mod tests;
