// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
#![allow(unused_variables)] // TODO remove

use libparsec_types::thiserror;
use libparsec_types::{AvailableDevice, DeviceAccessStrategy, DeviceLabel};

use crate::DeviceSaveStrategy;
use crate::Handle;

#[derive(Debug, thiserror::Error)]
pub enum ImportRecoveryDeviceError {}

#[derive(Debug, thiserror::Error)]
pub enum ExportRecoveryDeviceError {}

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
    access_strategy: DeviceAccessStrategy,
) -> Result<(String, Vec<u8>), ExportRecoveryDeviceError> {
    todo!()
}
