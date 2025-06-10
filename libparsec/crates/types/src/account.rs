// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use bytes::Bytes;
use serde::{Deserialize, Serialize};
use serde_with::*;

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
 * AccountVaultItemWebLocalDeviceKey
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(
    into = "AccountVaultItemWebLocalDeviceKeyData",
    from = "AccountVaultItemWebLocalDeviceKeyData"
)]
pub struct AccountVaultItemWebLocalDeviceKey {
    organization_id: OrganizationID,
    device_id: DeviceID,
    /// `SecretKey` encrypted by the vault key.
    /// This key is itself used to decrypt the `LocalDevice` stored on
    /// the web client's storage
    encrypted_data: Bytes,
}

parsec_data!("schema/account/account_vault_item_web_local_device_key.json5");

impl_transparent_data_format_conversion!(
    AccountVaultItemWebLocalDeviceKey,
    AccountVaultItemWebLocalDeviceKeyData,
    organization_id,
    device_id,
    encrypted_data,
);

impl_dump_and_encrypt!(AccountVaultItemWebLocalDeviceKey);
impl_decrypt_and_load!(AccountVaultItemWebLocalDeviceKey);

/*
 * AccountVaultItemRegistrationDevice
 */

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(
    into = "AccountVaultItemRegistrationDeviceData",
    from = "AccountVaultItemRegistrationDeviceData"
)]
pub struct AccountVaultItemRegistrationDevice {
    organization_id: OrganizationID,
    user_id: UserID,
    /// `LocalDevice` encrypted by the vault key
    encrypted_data: Bytes,
}

parsec_data!("schema/account/account_vault_item_auto_enrollment_device.json5");

impl_transparent_data_format_conversion!(
    AccountVaultItemRegistrationDevice,
    AccountVaultItemRegistrationDeviceData,
    organization_id,
    user_id,
    encrypted_data,
);

impl_dump_and_encrypt!(AccountVaultItemRegistrationDevice);
impl_decrypt_and_load!(AccountVaultItemRegistrationDevice);

#[cfg(test)]
#[path = "../tests/unit/account.rs"]
mod tests;
