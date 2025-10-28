// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

use libparsec_client::{ClientExportRecoveryDeviceError, ImportRecoveryDeviceError};
use libparsec_types::DeviceLabel;

use crate::{
    handle::{borrow_from_handle, Handle, HandleItem},
    AvailableDevice, ClientConfig, DeviceSaveStrategy,
};

pub async fn import_recovery_device(
    config: ClientConfig,
    recovery_device: &[u8],
    passphrase: String,
    device_label: DeviceLabel,
    save_strategy: DeviceSaveStrategy,
) -> Result<AvailableDevice, ImportRecoveryDeviceError> {
    let save_strategy = save_strategy.convert_with_side_effects()?;

    libparsec_client::import_recovery_device(
        &config.config_dir,
        recovery_device,
        passphrase,
        device_label,
        save_strategy,
    )
    .await
}

pub async fn client_export_recovery_device(
    client_handle: Handle,
    device_label: DeviceLabel,
) -> Result<(String, Vec<u8>), ClientExportRecoveryDeviceError> {
    let client = borrow_from_handle(client_handle, |x| match x {
        HandleItem::Client { client, .. } => Some(client.clone()),
        _ => None,
    })?;

    let (passphrase, data) = client.export_recovery_device(device_label).await?;

    Ok((passphrase.to_string(), data))
}
