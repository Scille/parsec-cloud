// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_client_connection::ConnectionError;
use libparsec_types::prelude::*;

use super::Account;

#[derive(Debug, thiserror::Error)]
pub enum AccountUploadDeviceFileAccountVaultKeyError {
    #[error("Cannot decrypt the vault key access return by the server: {0}")]
    BadVaultKeyAccess(DataError),
    #[error("Cannot communicate with the server: {0}")]
    Offline(#[from] ConnectionError),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

pub(super) async fn account_upload_device_file_account_vault_key(
    account: &Account,
    organization_id: OrganizationID,
    device_id: DeviceID,
    ciphertext_key: SecretKey,
) -> Result<(), AccountUploadDeviceFileAccountVaultKeyError> {
    // 1. Load the vault key

    let ciphered_key_access = {
        use libparsec_protocol::authenticated_account_cmds::latest::vault_item_list::{Rep, Req};

        let req = Req {};
        let rep = account.cmds.send(req).await?;

        match rep {
            Rep::Ok { key_access, .. } => key_access,
            bad_rep @ Rep::UnknownStatus { .. } => {
                return Err(anyhow::anyhow!("Unexpected server response: {:?}", bad_rep).into());
            }
        }
    };

    let vault_key = {
        let access = AccountVaultKeyAccess::decrypt_and_load(
            &ciphered_key_access,
            &account.auth_method_secret_key,
        )
        .map_err(AccountUploadDeviceFileAccountVaultKeyError::BadVaultKeyAccess)?;
        access.vault_key
    };

    // 2. Encrypt the key and save it in the account vault

    let encrypted_data = DeviceFileAccountVaultCiphertextKey { ciphertext_key }
        .dump_and_encrypt(&vault_key)
        .into();

    let item = AccountVaultItemDeviceFileKeyAccess {
        organization_id,
        device_id,
        encrypted_data,
    };

    {
        use libparsec_protocol::authenticated_account_cmds::v5::vault_item_upload::{Rep, Req};

        let req = Req {
            item_fingerprint: item.fingerprint(),
            item: item.dump().into(),
        };
        match account.cmds.send(req).await? {
            Rep::Ok => (),
            bad_rep @ (
                // Unexpected since the fingerprint is a hash containing, among
                // other things, the randomly chosen device ID.
                Rep::FingerprintAlreadyExists
                | Rep::UnknownStatus { .. }
            ) => {
                return Err(anyhow::anyhow!("Unexpected server response: {:?}", bad_rep).into());
            }
        }
    }

    Ok(())
}

#[cfg(test)]
#[path = "../tests/unit/upload_device_file_account_vault_key.rs"]
mod tests;
