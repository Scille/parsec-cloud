// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_client::RegisterNewDeviceError;
use libparsec_client_connection::{AuthenticatedCmds, ConnectionError};
use libparsec_platform_device_loader::{get_default_key_file, save_device, SaveDeviceError};
use libparsec_types::prelude::*;

use super::Account;

#[derive(Debug, thiserror::Error)]
pub enum AccountRegisterNewDeviceError {
    #[error("Not registration device exists for this organization/user IDs")]
    UnknownRegistrationDevice,
    #[error("Cannot communicate with the server: {0}")]
    Offline(#[from] ConnectionError),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
    #[error("Storage is not available")]
    StorageNotAvailable,
    #[error(transparent)]
    InvalidPath(anyhow::Error),
    #[error("Our clock ({client_timestamp}) and the server's one ({server_timestamp}) are too far apart")]
    TimestampOutOfBallpark {
        server_timestamp: DateTime,
        client_timestamp: DateTime,
        ballpark_client_early_offset: f64,
        ballpark_client_late_offset: f64,
    },
}

pub(super) async fn account_register_new_device(
    account: &Account,
    organization_id: OrganizationID,
    user_id: UserID,
    new_device_label: DeviceLabel,
    save_strategy: DeviceSaveStrategy,
) -> Result<AvailableDevice, AccountRegisterNewDeviceError> {
    // 1. Retrieve the registration device from the cache

    let registration_device = account
        .registration_devices_cache
        .lock()
        .expect("Mutex is poisoned")
        .iter()
        .find(|device| *device.organization_id() == organization_id && device.user_id == user_id)
        .ok_or(AccountRegisterNewDeviceError::UnknownRegistrationDevice)?
        .to_owned();

    // 2. Generate the new device (device ID, signing key etc.)

    let new_device =
        LocalDevice::from_existing_device_for_user(&registration_device, new_device_label);

    // We first register the new device to the server, and only then save it on disk.
    // This is to ensure the device found on disk are always valid, however the drawback
    // is it can lead to losing the device if something goes wrong between the two steps.
    //
    // This is considered acceptable given 1) the error window is small and
    // 2) if this occurs, the user can always re-import the recovery device.

    // 3. Upload the new device on the server

    let cmds = AuthenticatedCmds::from_client(
        account.cmds.client().to_owned(),
        &account.config_dir,
        registration_device.clone(),
    );
    libparsec_client::register_new_device(
        &cmds,
        &new_device,
        DevicePurpose::Standard,
        &registration_device,
    )
    .await
    .map_err(|err| match err {
        RegisterNewDeviceError::Offline(err) => AccountRegisterNewDeviceError::Offline(err),
        RegisterNewDeviceError::TimestampOutOfBallpark {
            server_timestamp,
            client_timestamp,
            ballpark_client_early_offset,
            ballpark_client_late_offset,
        } => AccountRegisterNewDeviceError::TimestampOutOfBallpark {
            server_timestamp,
            client_timestamp,
            ballpark_client_early_offset,
            ballpark_client_late_offset,
        },
        RegisterNewDeviceError::Internal(err) => AccountRegisterNewDeviceError::Internal(err),
    })?;

    // From now on, our new device has an actual existence !

    // 4. Save the new device on disk

    let access = {
        let key_file = get_default_key_file(&account.config_dir, new_device.device_id);
        save_strategy.into_access(key_file)
    };
    let new_available_device = save_device(&account.config_dir, &access, &new_device)
        .await
        .map_err(|err| match err {
            SaveDeviceError::StorageNotAvailable => {
                AccountRegisterNewDeviceError::StorageNotAvailable
            }
            SaveDeviceError::InvalidPath(error) => {
                AccountRegisterNewDeviceError::InvalidPath(error)
            }
            SaveDeviceError::Internal(error) => AccountRegisterNewDeviceError::Internal(error),
        })?;

    Ok(new_available_device)
}

#[cfg(test)]
#[path = "../tests/unit/register_new_device.rs"]
mod tests;
