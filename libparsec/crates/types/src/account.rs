// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use bytes::Bytes;
use libparsec_crypto::{HashDigest, SecretKey};
use rand::Rng;
use serde::{Deserialize, Serialize};
use serde_with::*;
use thiserror::Error;

use super::utils::{impl_decrypt_and_load, impl_dump, impl_dump_and_encrypt, impl_load};
use libparsec_serialization_format::parsec_data;

use crate::{
    self as libparsec_types,
    data_macros::impl_transparent_data_format_conversion,
    serialization::{format_v0_dump, format_vx_load},
    AccountVaultItemOpaqueKeyID, DataError, OrganizationID, UserID,
};

// The auth method master secret is the root secret from which are derived
// all other data used for authentication and end-2-end encryption:
// - ID & MAC key: used for server authentication.
// - Secret key: used for end-2-end encryption.
pub const AUTH_METHOD_ID_DERIVATION_UUID: uuid::Uuid =
    uuid::uuid!("00000000-0000-0000-0000-000000000000");
pub const AUTH_METHOD_MAC_KEY_DERIVATION_UUID: uuid::Uuid =
    uuid::uuid!("11111111-1111-1111-1111-111111111111");
pub const AUTH_METHOD_SECRET_KEY_DERIVATION_UUID: uuid::Uuid =
    uuid::uuid!("22222222-2222-2222-2222-222222222222");

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
    OpaqueKey(AccountVaultItemOpaqueKey),
    RegistrationDevice(AccountVaultItemRegistrationDevice),
}

impl AccountVaultItem {
    pub fn fingerprint(&self) -> HashDigest {
        match self {
            AccountVaultItem::OpaqueKey(item) => item.fingerprint(),
            AccountVaultItem::RegistrationDevice(item) => item.fingerprint(),
        }
    }
}

impl_load!(AccountVaultItem);

/*
 * AccountVaultItemOpaqueKey
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(
    into = "AccountVaultItemOpaqueKeyData",
    from = "AccountVaultItemOpaqueKeyData"
)]
pub struct AccountVaultItemOpaqueKey {
    pub key_id: AccountVaultItemOpaqueKeyID,
    /// `SecretKey` encrypted by the vault key.
    /// This key is itself used to decrypt the `LocalDevice` stored on
    /// the web client's storage
    pub encrypted_data: Bytes,
}

impl AccountVaultItemOpaqueKey {
    pub fn fingerprint(&self) -> HashDigest {
        // This format should not change in order to preserve compatibility with
        // the items already uploaded in the account vault.
        const LABEL: &str = "OPAQUE_KEY";
        HashDigest::from_data(format!("{}.{}", LABEL, self.key_id.hex()).as_bytes())
    }
}

parsec_data!("schema/account/account_vault_item_opaque_key.json5");

impl_transparent_data_format_conversion!(
    AccountVaultItemOpaqueKey,
    AccountVaultItemOpaqueKeyData,
    key_id,
    encrypted_data,
);

impl_dump!(AccountVaultItemOpaqueKey);

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

impl AccountVaultItemRegistrationDevice {
    pub fn fingerprint(&self) -> HashDigest {
        // This format should not change in order to preserve compatibility with
        // the items already uploaded in the account vault.
        const LABEL: &str = "REGISTRATION_DEVICE";
        HashDigest::from_data(
            format!("{}.{}.{}", LABEL, &self.organization_id, self.user_id.hex()).as_bytes(),
        )
    }
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

/*
 * AccountVaultItemOpaqueKeyEncryptedData
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(
    into = "AccountVaultItemOpaqueKeyEncryptedDataData",
    from = "AccountVaultItemOpaqueKeyEncryptedDataData"
)]
pub struct AccountVaultItemOpaqueKeyEncryptedData {
    pub key_id: AccountVaultItemOpaqueKeyID,
    pub key: SecretKey,
}

parsec_data!("schema/account/account_vault_item_opaque_key_encrypted_data.json5");

impl_transparent_data_format_conversion!(
    AccountVaultItemOpaqueKeyEncryptedData,
    AccountVaultItemOpaqueKeyEncryptedDataData,
    key_id,
    key,
);

impl_dump_and_encrypt!(AccountVaultItemOpaqueKeyEncryptedData);
impl_decrypt_and_load!(AccountVaultItemOpaqueKeyEncryptedData);

#[cfg(test)]
#[path = "../tests/unit/account.rs"]
mod tests;
