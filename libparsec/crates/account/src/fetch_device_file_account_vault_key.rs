// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_client_connection::ConnectionError;
use libparsec_types::prelude::*;

use super::{fetch_vault_items, Account, FetchVaultItemsError};

#[derive(Debug, thiserror::Error)]
pub enum AccountFetchDeviceFileAccountVaultKeyError {
    #[error("Cannot decrypt the vault key access return by the server: {0}")]
    BadVaultKeyAccess(DataError),
    #[error("No web local device key exists for this organization/device IDs")]
    UnknownWebLocalDeviceKey,
    #[error("The web local device key to be corrupted")]
    CorruptedWebLocalDeviceKey,
    #[error("Cannot communicate with the server: {0}")]
    Offline(#[from] ConnectionError),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

pub(super) async fn account_fetch_device_file_account_vault_key(
    account: &Account,
    organization_id: &OrganizationID,
    device_id: DeviceID,
) -> Result<SecretKey, AccountFetchDeviceFileAccountVaultKeyError> {
    // 1. Retrieve from the server the vault item containing the device file key access

    let (vault_key, vault_items) = fetch_vault_items(account).await.map_err(|err| match err {
        FetchVaultItemsError::BadVaultKeyAccess(err) => {
            AccountFetchDeviceFileAccountVaultKeyError::BadVaultKeyAccess(err)
        }
        FetchVaultItemsError::Offline(err) => {
            AccountFetchDeviceFileAccountVaultKeyError::Offline(err)
        }
        FetchVaultItemsError::Internal(err) => {
            AccountFetchDeviceFileAccountVaultKeyError::Internal(err)
        }
    })?;

    let encrypted_data = vault_items
        .into_iter()
        .find_map(|item| match item {
            AccountVaultItem::DeviceFileKeyAccess(item)
                if item.organization_id == *organization_id && item.device_id == device_id =>
            {
                Some(item.encrypted_data)
            }
            _ => None,
        })
        .ok_or(AccountFetchDeviceFileAccountVaultKeyError::UnknownWebLocalDeviceKey)?;

    // 2. Decrypt data with the vault key to finally obtain the device file key

    let key_access =
        DeviceFileAccountVaultCiphertextKey::decrypt_and_load(&encrypted_data, &vault_key)
            .map_err(|_| AccountFetchDeviceFileAccountVaultKeyError::CorruptedWebLocalDeviceKey)?;

    Ok(key_access.ciphertext_key)
}

#[cfg(test)]
#[path = "../tests/unit/fetch_device_file_account_vault_key.rs"]
mod tests;
