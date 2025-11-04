// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_client_connection::ConnectionError;
use libparsec_types::prelude::*;

use super::Account;

#[derive(Debug, thiserror::Error)]
pub enum AccountUploadOpaqueKeyInVaultError {
    #[error("Cannot decrypt the vault key access returned by the server: {0}")]
    BadVaultKeyAccess(DataError),
    #[error("Cannot communicate with the server: {0}")]
    Offline(#[from] ConnectionError),
    #[error("The Parsec account server returned an unexpected response: {0}")]
    BadServerResponse(anyhow::Error),
}

pub(super) async fn account_upload_opaque_key_in_vault(
    account: &Account,
) -> Result<(AccountVaultItemOpaqueKeyID, SecretKey), AccountUploadOpaqueKeyInVaultError> {
    // 1. Load the vault key

    let ciphered_key_access = {
        use libparsec_protocol::authenticated_account_cmds::latest::vault_item_list::{Rep, Req};

        let req = Req {};
        let rep = account.cmds.send(req).await?;

        match rep {
            Rep::Ok { key_access, .. } => key_access,
            bad_rep @ Rep::UnknownStatus { .. } => {
                return Err(AccountUploadOpaqueKeyInVaultError::BadServerResponse(
                    anyhow::anyhow!("{:?}", bad_rep),
                ));
            }
        }
    };

    let vault_key = {
        let access = AccountVaultKeyAccess::decrypt_and_load(
            &ciphered_key_access,
            &account.auth_method_secret_key,
        )
        .map_err(AccountUploadOpaqueKeyInVaultError::BadVaultKeyAccess)?;
        access.vault_key
    };

    // 2. Encrypt the key and save it in the account vault

    let key_id = AccountVaultItemOpaqueKeyID::default();
    let key = SecretKey::generate();

    let encrypted_data = AccountVaultItemOpaqueKeyEncryptedData {
        key_id,
        key: key.clone(),
    }
    .dump_and_encrypt(&vault_key)
    .into();

    let item = AccountVaultItemOpaqueKey {
        key_id,
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
                return Err(AccountUploadOpaqueKeyInVaultError::BadServerResponse(anyhow::anyhow!("{:?}", bad_rep)));
            }
        }
    }

    Ok((key_id, key))
}

#[cfg(test)]
#[path = "../tests/unit/upload_opaque_key_in_vault.rs"]
mod tests;
