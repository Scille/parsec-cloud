// Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS

use std::{path::Path, sync::Arc};

use libparsec_types::prelude::*;

use crate::{LoadDeviceError, LoadRecoveryDeviceError, SaveDeviceError, SaveRecoveryDeviceError};

/*
 * List available devices
 */

pub async fn list_available_devices(_config_dir: &Path) -> Vec<AvailableDevice> {
    todo!()
}

/*
 * Save & load
 */

pub async fn load_device(
    _access: &DeviceAccessStrategy,
) -> Result<Arc<LocalDevice>, LoadDeviceError> {
    todo!()
}

pub async fn save_device(
    _access: &DeviceAccessStrategy,
    _device: &LocalDevice,
) -> Result<(), SaveDeviceError> {
    todo!()
}

/*
 * Recovery
 */

pub async fn load_recovery_device(
    _key_file: &Path,
    _passphrase: SecretKeyPassphrase,
) -> Result<LocalDevice, LoadRecoveryDeviceError> {
    todo!()
}

pub async fn save_recovery_device(
    _key_file: &Path,
    _device: &LocalDevice,
) -> Result<SecretKeyPassphrase, SaveRecoveryDeviceError> {
    todo!()
}
