// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::collections::HashSet;

use libparsec_types::prelude::*;

use crate::FetchVaultItemsError;

use super::Account;

pub type AccountListRegistrationDevicesError = FetchVaultItemsError;

pub(super) async fn account_list_registration_devices(
    account: &Account,
) -> Result<HashSet<(OrganizationID, UserID)>, AccountListRegistrationDevicesError> {
    let (_, vault_items) = super::fetch_vault_items(account).await?;

    let items = vault_items
        .into_iter()
        .filter_map(|item| match item {
            AccountVaultItem::RegistrationDevice(item) => {
                Some((item.organization_id, item.user_id))
            }
            _ => None,
        })
        .collect();

    Ok(items)
}

#[cfg(test)]
#[path = "../tests/unit/list_registration_devices.rs"]
mod tests;
