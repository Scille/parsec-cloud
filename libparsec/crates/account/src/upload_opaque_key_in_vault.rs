// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_client_connection::ConnectionError;
use libparsec_types::prelude::*;

use super::Account;

#[derive(Debug, thiserror::Error)]
pub enum AccountUploadOpaqueKeyInVaultError {
    #[error(
        "The organization's configuration does not allow uploading sensitive data in the vault"
    )]
    NotAllowedByOrganizationVaultStrategy,
    #[error("The organization's configuration cannot be obtained (organization doesn't exist, or user not part of it ?")]
    CannotObtainOrganizationVaultStrategy,
    #[error("Cannot decrypt the vault key access return by the server: {0}")]
    BadVaultKeyAccess(DataError),
    #[error("Cannot communicate with the server: {0}")]
    Offline(#[from] ConnectionError),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

pub(super) async fn account_upload_opaque_key_in_vault(
    account: &Account,
    organization_id: &OrganizationID,
) -> Result<(AccountVaultItemOpaqueKeyID, SecretKey), AccountUploadOpaqueKeyInVaultError> {
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
        .map_err(AccountUploadOpaqueKeyInVaultError::BadVaultKeyAccess)?;
        access.vault_key
    };

    // 2. Check the organization's account vault strategy

    // TODO: Ugly temporary hack used by the GUI during organization bootstrap,
    // since this code needs a profound refactoring to be able to provide the
    // organization ID to this function...
    if organization_id.as_ref() != "__no_check__" {
        use libparsec_protocol::authenticated_account_cmds::latest::organization_self_list::{
            AccountVaultStrategy, Rep, Req,
        };

        let req = Req {};
        let rep = account.cmds.send(req).await?;

        let user = match rep {
            Rep::Ok { active, .. } => {
                let found = active
                    .into_iter()
                    .find(|u| u.organization_id == *organization_id);
                match found {
                    Some(user) => user,
                    // Multiple explanations here:
                    // - The organization doesn't exist
                    // - The organization exist, but the user is not part (or no longer) of it
                    //
                    // In theory those are exotic cases, since we most likely need an opaque vault key
                    // to upload a newly created device... however it can also be a bug (typically the
                    // provided organization ID doesn't match with the one from the newly created device).
                    None => {
                        return Err(AccountUploadOpaqueKeyInVaultError::CannotObtainOrganizationVaultStrategy);
                    }
                }
            }
            bad_rep @ Rep::UnknownStatus { .. } => {
                return Err(anyhow::anyhow!("Unexpected server response: {:?}", bad_rep).into());
            }
        };

        if matches!(
            user.organization_config.account_vault_strategy,
            AccountVaultStrategy::Forbidden
        ) {
            return Err(AccountUploadOpaqueKeyInVaultError::NotAllowedByOrganizationVaultStrategy);
        }
    }

    // 3. Encrypt the key and save it in the account vault

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
                return Err(anyhow::anyhow!("Unexpected server response: {:?}", bad_rep).into());
            }
        }
    }

    Ok((key_id, key))
}

#[cfg(test)]
#[path = "../tests/unit/upload_opaque_key_in_vault.rs"]
mod tests;
