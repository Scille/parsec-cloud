// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
#![allow(unused_variables)] // TODO remove

use std::sync::Arc;

use libparsec_client::{create_device_from_recovery, ProxyConfig};
pub use libparsec_client::{ExportRecoveryDeviceError, ImportRecoveryDeviceError};
use libparsec_client_connection::AuthenticatedCmds;
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
    let recovery_device: Arc<_> = libparsec_platform_device_loader::import_recovery_device(
        recovery_device,
        passphrase.into(),
    )
    .await?
    .into();

    let cmds = AuthenticatedCmds::new(
        &config.config_dir,
        recovery_device.clone(),
        ProxyConfig::default(),
    )?;

    let saved_device = create_device_from_recovery(
        cmds.into(),
        &recovery_device,
        &device_label,
        save_strategy,
        config.config_dir,
    )
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
