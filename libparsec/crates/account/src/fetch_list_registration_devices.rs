// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::{collections::HashSet, sync::Arc};

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
pub enum AccountFetchRegistrationDevicesError {
    #[error("Cannot decrypt the vault key access return by the server: {0}")]
    BadVaultKeyAccess(DataError),
    #[error("Cannot communicate with the server: {0}")]
    Offline(#[from] ConnectionError),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

pub(super) async fn account_fetch_registration_devices(
    account: &mut Account,
) -> Result<(), AccountFetchRegistrationDevicesError> {
    account.registration_devices_cache.clear();

    let (ciphered_key_access, items) = {
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
        .map_err(AccountFetchRegistrationDevicesError::BadVaultKeyAccess)?;
        access.vault_key
    };

    for (_, item_raw) in items {
        let item = match AccountVaultItem::load(&item_raw) {
            Ok(item) => item,
            Err(err) => {
                log::warn!("Cannot deserialize account vault item: {}", err);
                continue;
            }
        };
        let data = match item {
            AccountVaultItem::RegistrationDevice(data) => data,
            AccountVaultItem::WebLocalDeviceKey(_) => continue,
        };

        let mut registration_device = match LocalDevice::decrypt_and_load(
            &data.encrypted_data,
            &vault_key,
        ) {
            Ok(registration_device) => registration_device,
            Err(err) => {
                log::warn!(
                    "Cannot decrypt account vault registration device item (organization_id: {}, user_id: {}): {}",
                    data.organization_id,
                    data.user_id,
                    err
                );
                continue;
            }
        };

        registration_device.time_provider = account.time_provider.clone();

        account
            .registration_devices_cache
            .push(Arc::new(registration_device));
    }

    Ok(())
}

pub(super) fn account_list_registration_devices(
    account: &Account,
) -> HashSet<(OrganizationID, UserID)> {
    account
        .registration_devices_cache
        .iter()
        .map(|device| (device.organization_id().to_owned(), device.user_id))
        .collect()
}

#[cfg(test)]
#[path = "../tests/unit/fetch_list_registration_devices.rs"]
mod tests;
