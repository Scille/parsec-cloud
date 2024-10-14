// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
#![allow(unused_variables)] // TODO remove

use libparsec_platform_device_loader::{ExportRecoveryDeviceError, ImportRecoveryDeviceError};
use libparsec_types::DeviceSaveStrategy;
use libparsec_types::{AvailableDevice, DeviceAccessStrategy, DeviceLabel};

pub async fn import_recovery_device(
    recovery_device: Vec<u8>,
    passphrase: String,
    device_label: DeviceLabel,
    save_strategy: DeviceSaveStrategy,
) -> Result<AvailableDevice, ImportRecoveryDeviceError> {
    todo!()
}

pub async fn export_recovery_device(
    // client_handle: Handle,
    access_strategy: DeviceAccessStrategy,
) -> Result<(String, Vec<u8>), ExportRecoveryDeviceError> {
    todo!()
}
