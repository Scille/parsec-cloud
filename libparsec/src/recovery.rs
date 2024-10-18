// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
#![allow(unused_variables)] // TODO remove

use libparsec_client::{Client, EventBus};
pub use libparsec_client::{ExportRecoveryDeviceError, ImportRecoveryDeviceError};
use libparsec_platform_device_loader::get_default_key_file;
use libparsec_types::DeviceSaveStrategy;
use libparsec_types::{AvailableDevice, DeviceLabel};

use crate::handle::{borrow_from_handle, HandleItem};
use crate::ClientConfig;
use crate::Handle;

pub async fn import_recovery_device(
    config: ClientConfig,
    recovery_device: Vec<u8>,
    passphrase: String,
    device_label: DeviceLabel,
    save_strategy: DeviceSaveStrategy,
) -> Result<AvailableDevice, ImportRecoveryDeviceError> {
    let (recovery_device, new_device) =
        libparsec_platform_device_loader::inner_import_recovery_device(
            recovery_device,
            passphrase.into(),
            device_label,
        )
        .await?;
    let client = Client::start(
        config.clone().into(),
        EventBus::default(),
        recovery_device.into(),
    )
    .await?;
    let access = {
        let key_file = get_default_key_file(&config.config_dir, &new_device.device_id);
        save_strategy.into_access(key_file)
    };
    let saved_device = client
        .create_device_from_recovery(new_device, access)
        .await?;

    Ok(saved_device)
}

pub async fn export_recovery_device(
    client_handle: Handle,
    device_label: DeviceLabel,
) -> Result<(String, Vec<u8>), ExportRecoveryDeviceError> {
    let client = borrow_from_handle(client_handle, |x| match x {
        HandleItem::Client { client, .. } => Some(client.clone()),
        _ => None,
    })?;

    let (passphrase, data) = client.export_recovery_device(device_label).await?;

    Ok((passphrase.to_string(), data))
}
