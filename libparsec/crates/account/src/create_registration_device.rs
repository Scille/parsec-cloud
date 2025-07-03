// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use std::sync::Arc;

use libparsec_client::RegisterNewDeviceError;
use libparsec_client_connection::{AuthenticatedCmds, ConnectionError};
use libparsec_types::prelude::*;

use super::Account;

#[derive(Debug, thiserror::Error)]
pub enum AccountCreateRegistrationDeviceError {
    #[error("Cannot decrypt the vault key access return by the server: {0}")]
    BadVaultKeyAccess(DataError),
    #[error("Cannot communicate with the server: {0}")]
    Offline(#[from] ConnectionError),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
    #[error("Our clock ({client_timestamp}) and the server's one ({server_timestamp}) are too far apart")]
    TimestampOutOfBallpark {
        server_timestamp: DateTime,
        client_timestamp: DateTime,
        ballpark_client_early_offset: f64,
        ballpark_client_late_offset: f64,
    },
}

pub(super) async fn account_create_registration_device(
    account: &Account,
    existing_local_device: Arc<LocalDevice>,
) -> Result<DeviceID, AccountCreateRegistrationDeviceError> {
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
        .map_err(AccountCreateRegistrationDeviceError::BadVaultKeyAccess)?;
        access.vault_key
    };

    // 2. Generate the new device (device ID, signing key etc.)

    let new_device_label = "Registration device".parse().expect("valid");
    let new_device = Arc::new(LocalDevice::from_existing_device_for_user(
        &existing_local_device,
        new_device_label,
    ));

    // We first register the new device to the server (i.e. upload a device certificate),
    // and only then save it in the account vault.
    // This is to ensure the device found in the vault are always valid, however the drawback
    // is it can lead to losing the device if something goes wrong between the two steps.
    //
    // This is considered acceptable given 1) the error window is small and
    // 2) if this occurs, the user can always re-create a registration device.

    // 3. Upload the new device on the server

    let cmds = AuthenticatedCmds::from_client(
        account.cmds.client().to_owned(),
        &account.config_dir,
        existing_local_device.clone(),
    );
    libparsec_client::register_new_device(
        &cmds,
        &new_device,
        DevicePurpose::Registration,
        &existing_local_device,
    )
    .await
    .map_err(|err| match err {
        RegisterNewDeviceError::Offline(err) => AccountCreateRegistrationDeviceError::Offline(err),
        RegisterNewDeviceError::TimestampOutOfBallpark {
            server_timestamp,
            client_timestamp,
            ballpark_client_early_offset,
            ballpark_client_late_offset,
        } => AccountCreateRegistrationDeviceError::TimestampOutOfBallpark {
            server_timestamp,
            client_timestamp,
            ballpark_client_early_offset,
            ballpark_client_late_offset,
        },
        RegisterNewDeviceError::Internal(err) => {
            AccountCreateRegistrationDeviceError::Internal(err)
        }
    })?;

    // From now on, our new device has an actual existence !

    // 4. Save the new device on the account vault

    let item = AccountVaultItemRegistrationDevice {
        organization_id: new_device.organization_id().to_owned(),
        user_id: new_device.user_id,
        encrypted_data: new_device.dump_and_encrypt(&vault_key).into(),
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

    // 5. Also populate the registration device in the cache without waiting for
    // a new `fetch_list_registration_device` since it's simple and convenient.

    let new_device_id = new_device.device_id;
    account
        .registration_devices_cache
        .lock()
        .expect("Mutex is poisoned")
        .push(new_device);

    Ok(new_device_id)
}

#[cfg(test)]
#[path = "../tests/unit/create_registration_device.rs"]
mod tests;
