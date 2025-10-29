// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_client_connection::ConnectionError;
use libparsec_types::prelude::*;

use super::Account;

pub struct AccountListRegistrationDeviceItem {
    pub organization_id: OrganizationID,
    pub user_id: UserID,
}

pub struct AccountListRegistrationDeviceItems {
    pub items: Vec<AccountListRegistrationDeviceItem>,
    pub bad_entries: Vec<()>,
}

#[derive(Debug, thiserror::Error)]
pub enum FetchVaultItemsError {
    #[error("Cannot decrypt the vault key access returned by the server: {0}")]
    BadVaultKeyAccess(DataError),
    #[error("Cannot communicate with the server: {0}")]
    Offline(#[from] ConnectionError),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

pub(super) async fn fetch_vault_items(
    account: &Account,
) -> Result<(SecretKey, Vec<AccountVaultItem>), FetchVaultItemsError> {
    let (ciphered_key_access, raw_items) = {
        use libparsec_protocol::authenticated_account_cmds::latest::vault_item_list::{Rep, Req};

        let req = Req {};
        let rep = account.cmds.send(req).await?;

        match rep {
            Rep::Ok { key_access, items } => (key_access, items),
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
        .map_err(FetchVaultItemsError::BadVaultKeyAccess)?;
        access.vault_key
    };

    // Deserialize items
    let vault_items = raw_items
        .into_iter()
        .filter_map(|(_, raw_item)| match AccountVaultItem::load(&raw_item) {
            Ok(item) => Some(item),
            Err(err) => {
                log::warn!("Cannot deserialize account vault item: {err}");
                None
            }
        })
        .collect();

    Ok((vault_key, vault_items))
}
