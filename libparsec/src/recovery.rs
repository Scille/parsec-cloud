// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
#![allow(unused_variables)] // TODO remove

use libparsec_platform_device_loader::{ExportRecoveryDeviceError, ImportRecoveryDeviceError};
use libparsec_types::DeviceSaveStrategy;
use libparsec_types::{AvailableDevice, DeviceLabel};

use crate::handle::{borrow_from_handle, HandleItem};
use crate::Handle;

pub async fn import_recovery_device(
    recovery_device: Vec<u8>,
    passphrase: String,
    device_label: DeviceLabel,
    save_strategy: DeviceSaveStrategy,
) -> Result<AvailableDevice, ImportRecoveryDeviceError> {
    todo!()
}

pub async fn export_recovery_device(
    client_handle: Handle,
) -> Result<(String, Vec<u8>), ExportRecoveryDeviceError> {
    let client = borrow_from_handle(client_handle, |x| match x {
        HandleItem::Client { client, .. } => Some(client.clone()),
        _ => None,
    })?;

    let (passphrase, data) = client.export_recovery_device().await?;

    Ok((passphrase.to_string(), data))
}
