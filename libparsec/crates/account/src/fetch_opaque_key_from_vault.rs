// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_client_connection::ConnectionError;
use libparsec_types::prelude::*;

use super::{fetch_vault_items, Account, FetchVaultItemsError};

#[derive(Debug, thiserror::Error)]
pub enum AccountFetchOpaqueKeyFromVaultError {
    #[error("Cannot decrypt the vault key access returned by the server: {0}")]
    BadVaultKeyAccess(DataError),
    #[error("No opaque key with this ID among the vault items")]
    UnknownOpaqueKey,
    #[error("The vault item containing this opaque key is corrupted")]
    CorruptedOpaqueKey,
    #[error("Cannot communicate with the server: {0}")]
    Offline(#[from] ConnectionError),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

pub(super) async fn account_fetch_opaque_key_from_vault(
    account: &Account,
    key_id: AccountVaultItemOpaqueKeyID,
) -> Result<SecretKey, AccountFetchOpaqueKeyFromVaultError> {
    // 1. Retrieve from the server the vault item containing the opaque key

    let (vault_key, vault_items) = fetch_vault_items(account).await.map_err(|err| match err {
        FetchVaultItemsError::BadVaultKeyAccess(err) => {
            AccountFetchOpaqueKeyFromVaultError::BadVaultKeyAccess(err)
        }
        FetchVaultItemsError::Offline(err) => AccountFetchOpaqueKeyFromVaultError::Offline(err),
        FetchVaultItemsError::Internal(err) => AccountFetchOpaqueKeyFromVaultError::Internal(err),
    })?;

    let encrypted_data = vault_items
        .into_iter()
        .find_map(|item| match item {
            AccountVaultItem::OpaqueKey(item) if item.key_id == key_id => Some(item.encrypted_data),
            _ => None,
        })
        .ok_or(AccountFetchOpaqueKeyFromVaultError::UnknownOpaqueKey)?;

    // 2. Decrypt data with the vault key to finally obtain the device file key

    let key_access =
        AccountVaultItemOpaqueKeyEncryptedData::decrypt_and_load(&encrypted_data, &vault_key)
            .map_err(|_| AccountFetchOpaqueKeyFromVaultError::CorruptedOpaqueKey)?;

    if key_access.key_id != key_id {
        return Err(AccountFetchOpaqueKeyFromVaultError::CorruptedOpaqueKey);
    }

    Ok(key_access.key)
}

#[cfg(test)]
#[path = "../tests/unit/fetch_opaque_key_from_vault.rs"]
mod tests;
